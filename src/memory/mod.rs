pub trait MemoryAccess {
    fn read_u32(&self, address: u32) -> u32;
    fn write_u32(&mut self, address: u32, value: u32);

    fn read_u16(&self, address: u32) -> u16;
    fn write_u16(&mut self, address: u32, value: u16);

    fn read_u8(&self, address: u32) -> u8;
    fn write_u8(&mut self, address: u32, value: u8);
}

pub struct Memory {
    data: Vec<u8>, // Memory is represented as a vector of bytes
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }
}

impl MemoryAccess for Memory {
    fn read_u32(&self, address: u32) -> u32 {
        let addr = address as usize;
        let value = (self.data[addr] as u32)
            | ((self.data[addr + 1] as u32) << 8)
            | ((self.data[addr + 2] as u32) << 16)
            | ((self.data[addr + 3] as u32) << 24);
        value
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let addr = address as usize;
        self.data[addr] = (value & 0xFF) as u8;
        self.data[addr + 1] = ((value >> 8) & 0xFF) as u8;
        self.data[addr + 2] = ((value >> 16) & 0xFF) as u8;
        self.data[addr + 3] = ((value >> 24) & 0xFF) as u8;
    }
    
    fn read_u8(&self, address: u32) -> u8 {
        todo!()
    }
    
    fn write_u8(&mut self, address: u32, value: u8) {
        todo!()
    }
}