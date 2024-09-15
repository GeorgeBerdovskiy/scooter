#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

use super::{Index, Map};

/// Serves as a generic pool throughout the Scooter compiler.
#[derive(Clone)]
pub struct Pool<T: Clone + Eq + Hash> {
    /// List of values.
    values: Vec<T>,

    /// Map from values to their indices.
    lookup: HashMap<T, Index>,
}

impl<T: Clone + Eq + Hash> Pool<T> {
    /// Create an empty pool.
    pub fn new() -> Self {
        Pool {
            values: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    /// Insert a value into the pool (if it doesn't exit yet) and return its index.
    pub fn insert(&mut self, value: T) -> Index {
        match self.lookup.get(&value) {
            Some(index) => *index,
            None => {
                let index = self.values.len();

                self.values.push(value.clone());
                self.lookup.insert(value, index);

                index
            }
        }
    }

    /// Given a value, return its corresponding index.
    pub fn index_of(&self, value: &T) -> Option<&Index> {
        self.lookup.get(&value)
    }

    /// Given an index, return its corresponding value.
    pub fn value_of(&self, index: Index) -> Option<&T> {
        self.values.get(index)
    }
}
