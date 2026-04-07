use std::fs::File;
use std::io::Read;

use crate::cpu::DEBUG_PRINT;

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
    rom: Vec<u8>,
    pub vram: Vec<u8>,
    pub palette: Vec<u8>,
    wram: Vec<u8>,
    iwram: Vec<u8>,
    io_registers: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom: Vec::new(),
            vram: vec![0; 96 * 1024],
            palette: vec![0; 1024],
            wram: vec![0; 32 * 1024],
            iwram: vec![0; 4 * 1024],
            io_registers: vec![0; 256],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        if buffer.len() > 32 * 1024 * 1024 {
            return Err("ROM too large".to_string());
        }

        self.rom = buffer;
        Ok(())
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.vram
    }

    pub fn get_palette(&self) -> &[u8] {
        &self.palette
    }

    pub fn get_rom_size(&self) -> usize {
        self.rom.len()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryAccess for Memory {
    fn read_u32(&self, address: u32) -> u32 {
        let addr = address & 0x0FFFFFFF;

        match addr {
            0x00000000..=0x01FFFFFF => {
                let offset = addr as usize;
                if offset + 4 <= self.rom.len() {
                    (self.rom[offset] as u32)
                        | ((self.rom[offset + 1] as u32) << 8)
                        | ((self.rom[offset + 2] as u32) << 16)
                        | ((self.rom[offset + 3] as u32) << 24)
                } else {
                    0
                }
            }
            0x02000000..=0x0203FFFF => {
                let offset = (addr - 0x02000000) as usize;
                if offset + 4 <= self.iwram.len() {
                    (self.iwram[offset] as u32)
                        | ((self.iwram[offset + 1] as u32) << 8)
                        | ((self.iwram[offset + 2] as u32) << 16)
                        | ((self.iwram[offset + 3] as u32) << 24)
                } else {
                    0
                }
            }
            0x03000000..=0x03007FFF => {
                let offset = (addr - 0x03000000) as usize;
                if offset + 4 <= self.iwram.len() {
                    (self.iwram[offset] as u32)
                        | ((self.iwram[offset + 1] as u32) << 8)
                        | ((self.iwram[offset + 2] as u32) << 16)
                        | ((self.iwram[offset + 3] as u32) << 24)
                } else {
                    0
                }
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                (self.io_registers[offset] as u32)
                    | ((self.io_registers[offset + 1] as u32) << 8)
                    | ((self.io_registers[offset + 2] as u32) << 16)
                    | ((self.io_registers[offset + 3] as u32) << 24)
            }
            0x05000000..=0x050003FF => {
                let offset = (addr - 0x05000000) as usize;
                (self.palette[offset] as u32)
                    | ((self.palette[offset + 1] as u32) << 8)
                    | ((self.palette[offset + 2] as u32) << 16)
                    | ((self.palette[offset + 3] as u32) << 24)
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                if offset + 4 <= self.vram.len() {
                    (self.vram[offset] as u32)
                        | ((self.vram[offset + 1] as u32) << 8)
                        | ((self.vram[offset + 2] as u32) << 16)
                        | ((self.vram[offset + 3] as u32) << 24)
                } else {
                    0
                }
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                (self.vram[offset] as u32)
                    | ((self.vram[offset + 1] as u32) << 8)
                    | ((self.vram[offset + 2] as u32) << 16)
                    | ((self.vram[offset + 3] as u32) << 24)
            }
            0x08000000..=0x09FFFFFF => {
                let offset = (addr - 0x08000000) as usize;
                if offset + 4 <= self.rom.len() {
                    (self.rom[offset] as u32)
                        | ((self.rom[offset + 1] as u32) << 8)
                        | ((self.rom[offset + 2] as u32) << 16)
                        | ((self.rom[offset + 3] as u32) << 24)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let addr = address & 0x0FFFFFFF;

        match addr {
            0x02000000..=0x0203FFFF => {
                let offset = (addr - 0x02000000) as usize;
                self.iwram[offset] = (value & 0xFF) as u8;
                self.iwram[offset + 1] = ((value >> 8) & 0xFF) as u8;
                self.iwram[offset + 2] = ((value >> 16) & 0xFF) as u8;
                self.iwram[offset + 3] = ((value >> 24) & 0xFF) as u8;
            }
            0x03000000..=0x03007FFF => {
                let offset = (addr - 0x03000000) as usize;
                self.iwram[offset] = (value & 0xFF) as u8;
                self.iwram[offset + 1] = ((value >> 8) & 0xFF) as u8;
                self.iwram[offset + 2] = ((value >> 16) & 0xFF) as u8;
                self.iwram[offset + 3] = ((value >> 24) & 0xFF) as u8;
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                self.io_registers[offset] = (value & 0xFF) as u8;
                self.io_registers[offset + 1] = ((value >> 8) & 0xFF) as u8;
                self.io_registers[offset + 2] = ((value >> 16) & 0xFF) as u8;
                self.io_registers[offset + 3] = ((value >> 24) & 0xFF) as u8;
            }
            0x05000000..=0x050003FF => {
                let offset = (addr - 0x05000000) as usize;
                self.palette[offset] = (value & 0xFF) as u8;
                self.palette[offset + 1] = ((value >> 8) & 0xFF) as u8;
                self.palette[offset + 2] = ((value >> 16) & 0xFF) as u8;
                self.palette[offset + 3] = ((value >> 24) & 0xFF) as u8;
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                self.vram[offset] = (value & 0xFF) as u8;
                self.vram[offset + 1] = ((value >> 8) & 0xFF) as u8;
                self.vram[offset + 2] = ((value >> 16) & 0xFF) as u8;
                self.vram[offset + 3] = ((value >> 24) & 0xFF) as u8;
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                self.vram[offset] = (value & 0xFF) as u8;
                self.vram[offset + 1] = ((value >> 8) & 0xFF) as u8;
                self.vram[offset + 2] = ((value >> 16) & 0xFF) as u8;
                self.vram[offset + 3] = ((value >> 24) & 0xFF) as u8;
            }
            _ => {}
        }
    }

    fn read_u16(&self, address: u32) -> u16 {
        let addr = address & 0x0FFFFFFF;

        match addr {
            0x00000000..=0x01FFFFFF => {
                let offset = addr as usize;
                if offset + 2 <= self.rom.len() {
                    (self.rom[offset] as u16) | ((self.rom[offset + 1] as u16) << 8)
                } else {
                    0
                }
            }
            0x02000000..=0x0203FFFF => {
                let offset = (addr - 0x02000000) as usize;
                (self.iwram[offset] as u16) | ((self.iwram[offset + 1] as u16) << 8)
            }
            0x03000000..=0x03007FFF => {
                let offset = (addr - 0x03000000) as usize;
                (self.iwram[offset] as u16) | ((self.iwram[offset + 1] as u16) << 8)
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                (self.io_registers[offset] as u16) | ((self.io_registers[offset + 1] as u16) << 8)
            }
            0x05000000..=0x050003FF => {
                let offset = (addr - 0x05000000) as usize;
                (self.palette[offset] as u16) | ((self.palette[offset + 1] as u16) << 8)
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                (self.vram[offset] as u16) | ((self.vram[offset + 1] as u16) << 8)
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                (self.vram[offset] as u16) | ((self.vram[offset + 1] as u16) << 8)
            }
            0x08000000..=0x09FFFFFF => {
                let offset = (addr - 0x08000000) as usize;
                if offset + 2 <= self.rom.len() {
                    (self.rom[offset] as u16) | ((self.rom[offset + 1] as u16) << 8)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write_u16(&mut self, address: u32, value: u16) {
        let addr = address & 0x0FFFFFFF;

        if addr >= 0x06000000 && addr < 0x06018000 {
            let offset = (addr - 0x06000000) as usize;
            if DEBUG_PRINT.load(std::sync::atomic::Ordering::SeqCst)
                && crate::cpu::DEBUG_COUNT.load(std::sync::atomic::Ordering::SeqCst) < 100
            {
                eprintln!("WRITE VRAM[0x{:05X}] = 0x{:04X}", offset, value);
            }
        }

        match addr {
            0x02000000..=0x0203FFFF => {
                let offset = (addr - 0x02000000) as usize;
                self.iwram[offset] = (value & 0xFF) as u8;
                self.iwram[offset + 1] = ((value >> 8) & 0xFF) as u8;
            }
            0x03000000..=0x03007FFF => {
                let offset = (addr - 0x03000000) as usize;
                self.iwram[offset] = (value & 0xFF) as u8;
                self.iwram[offset + 1] = ((value >> 8) & 0xFF) as u8;
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                self.io_registers[offset] = (value & 0xFF) as u8;
                self.io_registers[offset + 1] = ((value >> 8) & 0xFF) as u8;
            }
            0x05000000..=0x050003FF => {
                let offset = (addr - 0x05000000) as usize;
                self.palette[offset] = (value & 0xFF) as u8;
                self.palette[offset + 1] = ((value >> 8) & 0xFF) as u8;
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                if offset < self.vram.len()
                    && DEBUG_PRINT.load(std::sync::atomic::Ordering::SeqCst)
                    && crate::cpu::DEBUG_COUNT.load(std::sync::atomic::Ordering::SeqCst) < 100
                {
                    eprintln!("WRITE VRAM[0x{:04X}] = 0x{:04X}", offset, value);
                }
                self.vram[offset] = (value & 0xFF) as u8;
                self.vram[offset + 1] = ((value >> 8) & 0xFF) as u8;
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                self.vram[offset] = (value & 0xFF) as u8;
                self.vram[offset + 1] = ((value >> 8) & 0xFF) as u8;
            }
            _ => {}
        }
    }

    fn read_u8(&self, address: u32) -> u8 {
        let addr = address & 0x0FFFFFFF;

        match addr {
            0x00000000..=0x01FFFFFF => {
                let offset = addr as usize;
                if offset < self.rom.len() {
                    self.rom[offset]
                } else {
                    0
                }
            }
            0x02000000..=0x0203FFFF => self.iwram[(addr - 0x02000000) as usize],
            0x03000000..=0x03007FFF => self.iwram[(addr - 0x03000000) as usize],
            0x04000000..=0x040000FF => self.io_registers[(addr - 0x04000000) as usize],
            0x05000000..=0x050003FF => self.palette[(addr - 0x05000000) as usize],
            0x06000000..=0x06017FFF => self.vram[(addr - 0x06000000) as usize],
            0x07000000..=0x070003FF => self.vram[(addr - 0x07000000) as usize],
            0x08000000..=0x09FFFFFF => {
                let offset = (addr - 0x08000000) as usize;
                if offset < self.rom.len() {
                    self.rom[offset]
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write_u8(&mut self, address: u32, value: u8) {
        let addr = address & 0x0FFFFFFF;

        match addr {
            0x02000000..=0x0203FFFF => {
                self.iwram[(addr - 0x02000000) as usize] = value;
            }
            0x03000000..=0x03007FFF => {
                self.iwram[(addr - 0x03000000) as usize] = value;
            }
            0x04000000..=0x040000FF => {
                self.io_registers[(addr - 0x04000000) as usize] = value;
            }
            0x05000000..=0x050003FF => {
                self.palette[(addr - 0x05000000) as usize] = value;
            }
            0x06000000..=0x06017FFF => {
                self.vram[(addr - 0x06000000) as usize] = value;
            }
            0x07000000..=0x070003FF => {
                self.vram[(addr - 0x07000000) as usize] = value;
            }
            _ => {}
        }
    }
}
