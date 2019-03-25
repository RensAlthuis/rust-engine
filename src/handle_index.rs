pub type Handle = usize;

enum Entry {
    Value(usize),
    Empty(usize)
}

pub struct HandleVec<T>{
    values : Vec<T>,
    handles : Vec<Entry>,
    next : Handle
}

impl<T> HandleVec<T>{
    pub fn new() -> HandleVec<T> {
        HandleVec {
            values : Vec::new(),
            handles : vec![Entry::Empty(1)],
            next : 0
        }
    }

    pub fn get(&self, handle : Handle) -> Option<&T> {
        if let Some(&Entry::Value(index)) = self.handles.get(handle){
            self.values.get(index)
        } else {
            None
        }
    }

    pub fn insert(&mut self, value : T) -> Handle {

        self.values.push(value);
        let index = self.values.len()-1;

        let res = self.next; //res is the newly returned handle
        self.next = match self.handles[res] { Entry::Empty(index) => index,
                                              Entry::Value(_) => panic!("handle_index found Value instead of Empty") }; //next becomes the next open handle

        self.handles[res] = Entry::Value(index);

        if self.next >= self.handles.len() {
            self.handles.push(Entry::Empty(self.handles.len()+1));
        }

        res
    }
}