use crate::{cpu::CPU, memory::{Memory, MemoryAccess}};

// use eframe::egui;
mod cpu;
mod memory;

fn main() {
    let memory: Box<dyn MemoryAccess> = Box::new(Memory::new(1024 * 1024)); // 1MB of memory
    let mut cpu = CPU::new(memory);

    let mut memory: [u32; 256] = [0; 256];
    memory[0] = 0xE0000000;

    for _ in 0..5 {
        cpu.fetch_decode_execute(&mut memory);
    }
}