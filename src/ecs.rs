
use typemap::Key;
use typemap::TypeMap;

use crate::handle_index::{Handle, HandleVec};

pub trait Component {}

struct Entity(TypeMap);
impl Entity {
    fn new() -> Entity {
        Entity(TypeMap::new())
    }
}

struct ComponentRegister<T: Component>(std::marker::PhantomData<T>);
impl<T: Component + 'static> Key for ComponentRegister<T> {
    type Value = HandleVec<T>;
}

struct ComponentEntry<T: Component>(std::marker::PhantomData<T>);
impl<T: Component + 'static> Key for ComponentEntry<T> {
    type Value = Handle;
}

pub struct Ecs {
    components: TypeMap,
    entities: Vec<Entity>,
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            components: TypeMap::new(),
            entities: Vec::new(),
        }
    }

    fn register_type<T: Component + 'static>(&mut self) {
        self.components.insert::<ComponentRegister<T>>(HandleVec::new());
    }

    fn create_comp<T: Component + 'static>(&mut self, comp: T) -> Option<Handle> {
        if let Some(store) = self.components.get_mut::<ComponentRegister<T>>() {
            Some(store.insert(comp))
        } else {
            None
        }
    }

    pub fn create_entity(&mut self) -> Handle {
        self.entities.push(Entity::new());
        self.entities.len() - 1
    }

    pub fn get_comp<T: Component + 'static>(&self, index: Handle) -> Option<&T> {
        if let Some(store) = self.components.get::<ComponentRegister<T>>() {
            store.get(index)
        } else {
            None
        }
    }

    pub fn add_comp<T: Component + 'static>(&mut self, entity: Handle, comp: T) -> bool {
        if !self.components.contains::<ComponentRegister<T>>() {
            self.register_type::<T>();
        }

        let index = self.create_comp(comp).unwrap();

        if let Some(entity) = self.entities.get_mut(entity) {
            if let None = entity.0.insert::<ComponentEntry<T>>(index) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}