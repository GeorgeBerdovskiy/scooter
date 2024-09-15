#![allow(dead_code)]

use std::hash::Hash;

use super::{Index, Map};

pub struct Interner<T: Clone + Eq + Hash> {
    map: Map<T, Index>,
    next: Index,
}

impl<T: Clone + Eq + Hash> Interner<T> {
    pub fn new() -> Self {
        Interner {
            map: Map::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, from: T) -> Index {
        if let Some(index) = self.map.from(&from) {
            return *index;
        }

        let index = self.next;

        self.map.insert(from, index);
        self.next += 1;

        index
    }

    pub fn index_of(&mut self, value: T) -> Option<&Index> {
        self.map.from(&value)
    }

    pub fn value_of(&mut self, index: Index) -> Option<&T> {
        self.map.to(&index)
    }
}
