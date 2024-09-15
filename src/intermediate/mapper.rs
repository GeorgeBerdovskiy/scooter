use super::{table::SymbolTable, Index};

pub struct Mapper<'a> {
    /// Internal map from indices to Ts.
    table: SymbolTable<'a, Index>,

    /// Next available index.
    pub next: Index,
}

impl<'a> Mapper<'a> {
    pub fn new() -> Self {
        Mapper {
            table: SymbolTable::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, value: &'a str) -> Index {
        let index = self.next;

        self.table.insert(value, index);
        self.next += 1;

        index
    }

    pub fn find(&mut self, value: &'a str) -> Index {
        self.table.find(value).unwrap()
    }

    pub fn up(&mut self) {
        let prev = self.table.clone();
        self.table = SymbolTable::new().with_previous(prev);
    }

    pub fn down(&mut self) {
        self.table = *self.table.previous.clone().unwrap();
    }
}
