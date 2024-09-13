use std::collections::HashMap;

use super::Index;

pub struct Mapper<T> {
    /// Internal map from indices to Ts.
    map: HashMap<Index, T>,

    /// Next available index.
    next: Index,
}

impl<T> Mapper<T> {
    pub fn new() -> Self {
        Mapper {
            map: HashMap::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> Index {
        let index = self.next;

        self.map.insert(index, value);
        self.next += 1;

        index
    }
}
