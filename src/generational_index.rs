#[derive(Debug, Clone)]
pub struct Index{
    index : usize,
    generation : usize
}

enum Entry {
    Live(usize),
    Dead(usize)
}

impl Entry {
    fn is_live(&self) -> bool {
        match self {
            Entry::Live(_) => true,
            Entry::Dead(_) => false
        }
    }
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

        let first = self.generations.iter_mut()
                            .skip_while(|x| { x.is_live() } )
                            .enumerate()
                            .next();

        if let Some((index,entry)) = first {
            match entry {
                &mut Entry::Dead(gen) => {
                    *entry = Entry::Live(gen);
                    Index {
                        index : index,
                        generation : gen+1
                    }
                },
                _ => panic!("Found Live entry in get function")
            }
        }else{
            self.generations.push(Entry::Live(0));
            Index{
                index:self.generations.len()-1,
                generation:0
            }
        }
    }

    pub fn release(&mut self, index : Index) -> bool {
        let Index {index, generation} = index;
        self.generations.get_mut(index).map_or_else(
            || {false},
            |entry| {
                if let Entry::Live(gen) = entry{
                    if *gen == generation {
                        *entry = Entry::Dead(*gen);
                        true
                    } else {
                        false
                    }
                }else{
                    false
                }
            }
        )
        // if let Some(Entry::Live(gen)) = self.generations.get(index) {
        //     if *gen == generation {
        //         self.generations[index] = Entry::Dead(*gen);
        //         true
        //     } else {
        //         false
        //     }
        // }
        // else {
        //     false
        // }
    }

    pub fn is_live(&self, index : Index) -> bool {
        let Index {index, generation:_} = index;
        self.generations.get(index)
                        .map_or_else(
                            | | {false},
                            |x| {x.is_live()})
    }
}