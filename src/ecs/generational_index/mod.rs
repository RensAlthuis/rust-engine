mod index;
pub use index::Index;

pub struct GIVec<T> {
    allocator : index::Allocator,
    vec : Vec<T>
}

impl<T> GIVec<T> {
    pub fn new() -> GIVec<T> {
        GIVec {
            allocator : index::Allocator::new(),
            vec : Vec::new()
        }
    }

    pub fn insert(&mut self, element : T) -> Index {
        let index = self.allocator.get();
        if let Some(val) = self.vec.get_mut(index.index) {
            *val = element;
        }else {
            self.vec.insert(index.index, element);
        }

        index
    }

    pub fn get(&self, index : &Index) -> Option<&T> {
        if !self.allocator.is_live(index){
            None
        }else{
            self.vec.get(index.index)
        }
    }

    pub fn get_mut(&mut self, index : &Index) -> Option<&mut T> {
        if !self.allocator.is_live(index){
            None
        }else{
            self.vec.get_mut(index.index)
        }
    }

    pub fn remove(&mut self, index : Index) -> bool {
        self.allocator.release(index)
    }
}