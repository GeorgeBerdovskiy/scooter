use super::{table::SymbolTable, Index};

/// Maps identifiers for variables and functions to unique indices.
pub struct Mapper<'a> {
    /// Internal map from indices to Ts.
    table: SymbolTable<'a, Index>,

    /// Next available index.
    pub next: Index,
}

impl<'a> Mapper<'a> {
    /// Create a new empty map.
    pub fn new() -> Self {
        Mapper {
            table: SymbolTable::new(),
            next: 0,
        }
    }

    /// Insert a new value into the map and return its unique index.
    pub fn insert(&mut self, value: &'a str) -> Index {
        let index = self.next;

        self.table.insert(value, index);
        self.next += 1;

        index
    }

    /// Given a value, find its unique index.
    pub fn find(&mut self, value: &'a str) -> Index {
        self.table.find(value).unwrap()
    }

    /// Add one table to the stack of symbol tables.
    pub fn up(&mut self) {
        let prev = self.table.clone();
        self.table = SymbolTable::new().with_previous(prev);
    }

    /// Pop one table from the stack of symbol tables.
    pub fn down(&mut self) {
        self.table = *self.table.previous.clone().unwrap();
    }
}
