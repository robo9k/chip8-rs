//! Memory

use crate::instructions::Addr;

const RAM_SIZE: usize = 4096;
/// RAM
pub struct Memory {
    ram: [u8; RAM_SIZE],
}

impl Memory {
    /// Creates a new instance intialized with `0`
    pub fn new() -> Self {
        Self { ram: [0; RAM_SIZE] }
    }

    /// Reads a byte at `addr`
    pub fn read(&self, addr: Addr) -> u8 {
        let addr: usize = addr.into();
        self.ram[addr]
    }

    /// Writes a `val` byte at `addr`
    pub fn write(&mut self, addr: Addr, val: u8) {
        let addr: usize = addr.into();
        self.ram[addr] = val;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
