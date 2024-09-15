#![allow(dead_code)]

/// Manages registers of any kind.
pub struct RegMgr<const N: usize> {
    /// Array of booleans representing whether register `i` is free or not.
    registers: [bool; N],

    /// All free registers.
    free: Vec<usize>,
}

impl<const N: usize> RegMgr<N> {
    /// Given its index, checks whether a register is currently in use or not.
    pub fn is_free(&self, index: usize) -> bool {
        self.registers[index]
    }

    /// Returns the index of a free register, or `None` if they're all occupied.
    pub fn get_free(&mut self) -> Option<usize> {
        self.free.pop()
    }

    /// Given its index, sets a register as free.
    pub fn set_free(&mut self, index: usize) {
        if !self.registers[index] {
            self.registers[index] = true;
            self.free.push(index);
        }
    }

    /// Given its index, sets a register as used.
    pub fn set_used(&mut self, index: usize) {
        self.registers[index] = false;
        self.free.retain(|i| *i != index);
    }
}
