use crate::{cpu::CPU, memory::{Memory, MemoryAccess}};

// use eframe::egui;
mod cpu;
mod memory;

fn main() {
    let memory: Box<dyn MemoryAccess> = Box::new(Memory::new(1024 * 1024)); // 1MB of memory
    let mut cpu = CPU::new(memory);

    for _ in 0..5 {
        cpu.fetch_decode_execute();
    }
}