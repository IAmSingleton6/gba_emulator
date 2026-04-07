use std::fs::File;
use std::io::Read;

pub trait MemoryAccess {
    fn read_u32(&self, address: u32) -> u32;
    fn write_u32(&mut self, address: u32, value: u32);

    fn read_u16(&self, address: u32) -> u16;
    fn write_u16(&mut self, address: u32, value: u16);

    fn read_u8(&self, address: u32) -> u8;
    fn write_u8(&mut self, address: u32, value: u8);
}

#[derive(Clone)]
pub struct Memory {
    data: Vec<u8>,
    pub vram: Vec<u8>,
    pub palette: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: vec![0; 1024 * 1024 * 32],
            vram: Vec::new(),
            palette: Vec::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        if buffer.len() > self.data.len() {
            return Err("ROM too large".to_string());
        }

        self.data[..buffer.len()].copy_from_slice(&buffer);
        Ok(())
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.vram
    }

    pub fn get_palette(&self) -> &[u8] {
        &self.palette
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryAccess for Memory {
    fn read_u32(&self, address: u32) -> u32 {
        let addr = (address & 0x0FFFFFFF) as usize;
        if addr + 3 >= self.data.len() {
            return 0;
        }
        let value = (self.data[addr] as u32)
            | ((self.data[addr + 1] as u32) << 8)
            | ((self.data[addr + 2] as u32) << 16)
            | ((self.data[addr + 3] as u32) << 24);
        value
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let addr = (address & 0x0FFFFFFF) as usize;
        if addr + 3 >= self.data.len() {
            return;
        }
        self.data[addr] = (value & 0xFF) as u8;
        self.data[addr + 1] = ((value >> 8) & 0xFF) as u8;
        self.data[addr + 2] = ((value >> 16) & 0xFF) as u8;
        self.data[addr + 3] = ((value >> 24) & 0xFF) as u8;
    }

    fn read_u16(&self, address: u32) -> u16 {
        let addr = (address & 0x0FFFFFFF) as usize;
        if addr + 1 >= self.data.len() {
            return 0;
        }
        (self.data[addr] as u16) | ((self.data[addr + 1] as u16) << 8)
    }

    fn write_u16(&mut self, address: u32, value: u16) {
        let addr = (address & 0x0FFFFFFF) as usize;
        if addr + 1 >= self.data.len() {
            return;
        }
        self.data[addr] = (value & 0xFF) as u8;
        self.data[addr + 1] = ((value >> 8) & 0xFF) as u8;
    }

    fn read_u8(&self, address: u32) -> u8 {
        let addr = (address & 0x0FFFFFFF) as usize;
        if addr >= self.data.len() {
            return 0;
        }
        self.data[addr]
    }

    fn write_u8(&mut self, address: u32, value: u8) {
        let addr = (address & 0x0FFFFFFF) as usize;
        if addr >= self.data.len() {
            return;
        }
        self.data[addr] = value;
    }
}
