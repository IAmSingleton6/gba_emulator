use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

pub trait MemoryAccess {
    fn read_u32(&self, address: u32) -> u32;
    fn write_u32(&mut self, address: u32, value: u32);
    fn read_u16(&self, address: u32) -> u16;
    fn write_u16(&mut self, address: u32, value: u16);
    fn read_u8(&self, address: u32) -> u8;
    fn write_u8(&mut self, address: u32, value: u8);
}

impl MemoryAccess for Rc<RefCell<Memory>> {
    fn read_u32(&self, address: u32) -> u32 {
        self.borrow().read_u32(address)
    }
    fn write_u32(&mut self, address: u32, value: u32) {
        self.borrow_mut().write_u32(address, value)
    }
    fn read_u16(&self, address: u32) -> u16 {
        self.borrow().read_u16(address)
    }
    fn write_u16(&mut self, address: u32, value: u16) {
        self.borrow_mut().write_u16(address, value)
    }
    fn read_u8(&self, address: u32) -> u8 {
        self.borrow().read_u8(address)
    }
    fn write_u8(&mut self, address: u32, value: u8) {
        self.borrow_mut().write_u8(address, value)
    }
}

#[derive(Clone)]
pub struct Memory {
    rom: Vec<u8>,
    pub vram: Vec<u8>,
    pub palette: Vec<u8>,
    wram: Vec<u8>,
    iwram: Vec<u8>,
    io_registers: Vec<u8>,
    oam: Vec<u8>,
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
            oam: vec![0; 1024],
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
                    u32::from_le_bytes([
                        self.rom[offset],
                        self.rom[offset + 1],
                        self.rom[offset + 2],
                        self.rom[offset + 3],
                    ])
                } else {
                    0
                }
            }
            0x02000000..=0x0203FFFF | 0x03000000..=0x03007FFF => {
                let offset = (addr & 0x7FFF) as usize;
                if offset + 4 <= self.iwram.len() {
                    u32::from_le_bytes([
                        self.iwram[offset],
                        self.iwram[offset + 1],
                        self.iwram[offset + 2],
                        self.iwram[offset + 3],
                    ])
                } else {
                    0
                }
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                u32::from_le_bytes([
                    self.io_registers[offset],
                    self.io_registers[offset + 1],
                    self.io_registers[offset + 2],
                    self.io_registers[offset + 3],
                ])
            }
            0x05000000..=0x050003FF => {
                let offset = ((addr - 0x05000000) & 0x3FF) as usize;
                u32::from_le_bytes([
                    self.palette[offset],
                    self.palette[offset + 1],
                    self.palette[offset + 2],
                    self.palette[offset + 3],
                ])
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                if offset + 4 <= self.vram.len() {
                    u32::from_le_bytes([
                        self.vram[offset],
                        self.vram[offset + 1],
                        self.vram[offset + 2],
                        self.vram[offset + 3],
                    ])
                } else {
                    0
                }
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                u32::from_le_bytes([
                    self.vram[offset],
                    self.vram[offset + 1],
                    self.vram[offset + 2],
                    self.vram[offset + 3],
                ])
            }
            0x08000000..=0x09FFFFFF => {
                let offset = (addr - 0x08000000) as usize;
                if offset + 4 <= self.rom.len() {
                    u32::from_le_bytes([
                        self.rom[offset],
                        self.rom[offset + 1],
                        self.rom[offset + 2],
                        self.rom[offset + 3],
                    ])
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let addr = address & 0x0FFFFFFF;
        let bytes = value.to_le_bytes();

        // Handle palette mirror - any write to 0x05000000-0x05000FFF goes to palette
        if addr >= 0x05000000 && addr < 0x05001000 {
            let offset = ((addr - 0x05000000) & 0x3FF) as usize;
            if offset + 4 <= self.palette.len() {
                self.palette[offset..offset + 4].copy_from_slice(&bytes);
            }
            return;
        }

        match addr {
            0x02000000..=0x0203FFFF | 0x03000000..=0x03007FFF => {
                let offset = (addr & 0x7FFF) as usize;
                if offset + 4 <= self.iwram.len() {
                    self.iwram[offset..offset + 4].copy_from_slice(&bytes);
                }
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                self.io_registers[offset..offset + 4].copy_from_slice(&bytes);
            }
            0x05000000..=0x050003FF => {
                let offset = (addr - 0x05000000) as usize;
                self.palette[offset..offset + 4].copy_from_slice(&bytes);
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                if offset + 4 <= self.vram.len() {
                    self.vram[offset..offset + 4].copy_from_slice(&bytes);
                }
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                self.vram[offset..offset + 4].copy_from_slice(&bytes);
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
                    u16::from_le_bytes([self.rom[offset], self.rom[offset + 1]])
                } else {
                    0
                }
            }
            0x02000000..=0x0203FFFF | 0x03000000..=0x03007FFF => {
                let offset = (addr & 0x7FFF) as usize;
                u16::from_le_bytes([self.iwram[offset], self.iwram[offset + 1]])
            }
            0x04000000..=0x040000FF => {
                let offset = (addr - 0x04000000) as usize;
                u16::from_le_bytes([self.io_registers[offset], self.io_registers[offset + 1]])
            }
            0x05000000..=0x050003FF => {
                let offset = (addr - 0x05000000) as usize;
                u16::from_le_bytes([self.palette[offset], self.palette[offset + 1]])
            }
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                u16::from_le_bytes([self.vram[offset], self.vram[offset + 1]])
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                u16::from_le_bytes([self.vram[offset], self.vram[offset + 1]])
            }
            0x08000000..=0x09FFFFFF => {
                let offset = (addr - 0x08000000) as usize;
                if offset + 2 <= self.rom.len() {
                    u16::from_le_bytes([self.rom[offset], self.rom[offset + 1]])
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write_u16(&mut self, address: u32, value: u16) {
        let addr = address & 0x0FFFFFFF;
        let bytes = value.to_le_bytes();

        // Debug: log all write_u16 to VRAM
        if addr >= 0x06000000 && addr < 0x06018000 {
            eprintln!("write_u16 VRAM: addr=0x{:08X}, value=0x{:04X}", addr, value);
        }

        // Handle palette mirror - any write to 0x05000000-0x05000FFF goes to palette
        if addr >= 0x05000000 && addr < 0x05001000 {
            let offset = ((addr - 0x05000000) & 0x3FF) as usize;
            if offset + 1 < self.palette.len() {
                self.palette[offset] = bytes[0];
                self.palette[offset + 1] = bytes[1];
            }
            return;
        }

        match addr {
            0x06000000..=0x06017FFF => {
                let offset = (addr - 0x06000000) as usize;
                self.vram[offset] = bytes[0];
                self.vram[offset + 1] = bytes[1];
            }
            0x03000000..=0x03007FFF => {
                let offset = (addr - 0x03000000) as usize;
                self.iwram[offset] = bytes[0];
                self.iwram[offset + 1] = bytes[1];
            }
            0x04000000..=0x040003FF => {
                let offset = (addr - 0x04000000) as usize;
                self.io_registers[offset] = bytes[0];
                self.io_registers[offset + 1] = bytes[1];
            }
            0x07000000..=0x070003FF => {
                let offset = (addr - 0x07000000) as usize;
                if offset + 1 < self.oam.len() {
                    self.oam[offset] = bytes[0];
                    self.oam[offset + 1] = bytes[1];
                }
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
            0x02000000..=0x0203FFFF | 0x03000000..=0x03007FFF => {
                self.iwram[(addr & 0x7FFF) as usize]
            }
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

        // Handle palette mirror for writes too
        if addr >= 0x05000000 && addr < 0x05001000 {
            let offset = ((addr - 0x05000000) & 0x3FF) as usize;
            self.palette[offset] = value;
            return;
        }

        match addr {
            0x02000000..=0x0203FFFF | 0x03000000..=0x03007FFF => {
                self.iwram[(addr & 0x7FFF) as usize] = value;
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
