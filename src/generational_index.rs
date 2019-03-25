pub struct Index{
    index : usize,
    generation : usize
}

enum Entry {
    Live(usize),
    Dead(usize)
}

pub struct GenerationalIndexAllocator {
    generations : Vec<Entry>
}

impl GenerationalIndexAllocator {
    pub fn new() -> GenerationalIndexAllocator {
        GenerationalIndexAllocator {
            generations : Vec::new()
        }
    }

    pub fn get(&mut self) -> Index {
        let iter = self.generations.iter_mut();
        for (i, entry) in iter.enumerate() {
            match entry {
                &mut Entry::Dead(gen) => {
                    *entry = Entry::Live(gen+1);
                    return Index {
                        index : i,
                        generation : gen+1
                    }
                },
                _ => ()
            };
        }
        self.generations.push(Entry::Live(0));
        Index{index:self.generations.len()-1, generation:0}

    }

    pub fn release(&mut self, index : Index) {
        let Index {index, generation} = index;
        if let Some(Entry::Live(gen)) = self.generations.get(index) {
            if *gen == generation {
                self.generations[index] = Entry::Dead(*gen);
            }
        }
    }
}