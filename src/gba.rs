use crate::cpu::CPU;
use crate::memory::Memory;
use crate::ppu::PPU;

pub struct GBA {
    cpu: CPU,
    memory: Memory,
    ppu: PPU,
}

impl GBA {
    pub fn new() -> Self {
        GBA {
            cpu: CPU::new(),
            memory: Memory::new(),
            ppu: PPU::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), String> {
        self.memory.load_rom(path)?;
        self.cpu.initialize_gba();
        Ok(())
    }

    pub fn step(&mut self) {
        self.cpu.fetch_decode_execute(&mut self.memory);
    }

    pub fn run_frame(&mut self) {
        loop {
            let cycles = self.cpu.fetch_decode_execute(&mut self.memory);

            if self.ppu.step(&self.memory, cycles) {
                break;
            }
        }
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        self.ppu.get_framebuffer()
    }
}
