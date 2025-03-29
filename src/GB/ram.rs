pub const RAM_SIZE: u32 = 65536;

pub struct RAM {
    memory: [u8; RAM_SIZE as usize],
}

impl RAM {
    pub fn new() -> Self {
        RAM { memory: [0; RAM_SIZE as usize] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}