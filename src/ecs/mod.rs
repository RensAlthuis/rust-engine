
mod generational_index;
mod entity;


use entity::Entity;
use generational_index::GIVec;
use typemap::{Key, TypeMap};

type Handle = generational_index::Index;

pub trait Component {}

//Key registry for typemaps
struct ComponentRegister<T: Component>(std::marker::PhantomData<T>);
impl<T: Component + 'static> Key for ComponentRegister<T> {
    type Value = GIVec<T>;
}

struct ComponentEntry<T: Component>(std::marker::PhantomData<T>);
impl<T: Component + 'static> Key for ComponentEntry<T> {
    type Value = Handle;
}

//ECS
pub struct Ecs {
    components: TypeMap,
    entities: GIVec<Entity>,
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            components: TypeMap::new(),
            entities: GIVec::new(),
        }
    }

    fn register_type<T: Component + 'static>(&mut self) {
        self.components.insert::<ComponentRegister<T>>(GIVec::new());
    }

    fn create_comp<T: Component + 'static>(&mut self, comp: T) -> Option<Handle> {
        if let Some(store) = self.components.get_mut::<ComponentRegister<T>>() {
            Some(store.insert(comp))
        } else {
            None
        }
    }


    //PUBLIC METHODS
    pub fn create_entity(&mut self) -> Handle {
        self.entities.insert(Entity::new())
    }

    pub fn delete_entity(&mut self, handle : Handle) -> bool {
        self.entities.remove(handle)
    }

    pub fn get_comp<T: Component + 'static>(&self, index: &Handle) -> Option<&T> {
        self.components.get::<ComponentRegister<T>>().and_then(|store| {
            store.get(index)
        })
    }

    pub fn add_comp<T: Component + 'static>(&mut self, entity: &Handle, comp: T) -> bool {
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

    pub fn remove_comp<T: Component + 'static>(&mut self, entity: &Handle) -> bool {
        let comp_handle = self.entities.get_mut(entity).and_then(|x| {
            x.0.remove::<ComponentEntry<T>>()
        });

        let comp_handle = match comp_handle {None => {return false}, Some(val) => val };

        let register = self.components.get_mut::<ComponentRegister<T>>().expect("found component handle, but component doesn't exist");

        register.remove(comp_handle)
    }
}