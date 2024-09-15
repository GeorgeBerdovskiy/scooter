#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};

/// Generic bidirectional map used throughout the compiler.
pub struct Map<F: Eq + Hash, T: Eq + Hash> {
    forward: HashMap<F, T>,
    reverse: HashMap<T, F>,
}

impl<F: Clone + Eq + Hash, T: Clone + Eq + Hash> Map<F, T> {
    pub fn new() -> Self {
        Map {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, from: F, to: T) {
        self.forward.insert(from.clone(), to.clone());
        self.reverse.insert(to, from);
    }

    pub fn from(&mut self, from: &F) -> Option<&T> {
        self.forward.get(from)
    }

    pub fn to(&mut self, to: &T) -> Option<&F> {
        self.reverse.get(to)
    }

    pub fn change(&mut self, from: &F, to: T) {
        // Remove the old "to"
        self.reverse.remove(&self.forward[from]);

        // Set the new forward mapping
        self.forward.insert(from.clone(), to.clone());

        // Set the new reverse mapping
        self.reverse.insert(to.clone(), from.clone());
    }
}
