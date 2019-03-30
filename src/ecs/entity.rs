use typemap::TypeMap;

pub struct Entity(pub TypeMap);
impl Entity {
    pub fn new() -> Entity {
        Entity(TypeMap::new())
    }
}