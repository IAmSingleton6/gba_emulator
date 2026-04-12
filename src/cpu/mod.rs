use decoder::{decode_arm, decode_thumb};
use executor::{ArmExecutor, ThumbExecutor};

use crate::memory::Memory;
use crate::{cpu::registers::Registers, memory::MemoryAccess};

mod decoder;
pub mod executor;
mod operations;
mod registers;

pub static DEBUG_PRINT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub static DEBUG_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub struct CPU {
    registers: registers::Registers,
    total_cycles: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            total_cycles: 0,
        }
    }

    pub fn initialize_gba(&mut self) {
        self.registers.set_pc(0x08000000);
        self.registers.set_thumb_state(false);

        self.registers.set_r(13, 0x03007F00);
        self.registers.set_r(14, 0x08000000);

        // CPSR: N=0, Z=0, C=0, V=0, T=0 (ARM mode), Mode=0x10 (Supervisor)
        self.registers.set_cpsr(0x00000010);
    }

    pub fn fetch_decode_execute(&mut self, memory: &mut Memory) -> u64 {
        let is_in_thumb_mode: bool = self.is_in_thumb_mode();
        let pc: u32 = self.registers.get_pc();

        let opcode: u32 = self.fetch(memory, is_in_thumb_mode);

        let cycles = if is_in_thumb_mode {
            let executor: ThumbExecutor = decode_thumb(opcode as u16);
            executor(self, memory, opcode as u16)
        } else {
            let executor: ArmExecutor = decode_arm(opcode);
            executor(self, memory, opcode)
        };

        self.total_cycles += cycles;
        cycles
    }

    pub fn fetch(&mut self, memory: &mut Memory, is_in_thumb_mode: bool) -> u32 {
        let pc: u32 = self.registers.get_pc();
        let instruction = memory.read_u32(pc);
        self.registers
            .set_pc(pc.wrapping_add(if is_in_thumb_mode { 2 } else { 4 }));
        instruction
    }

    pub fn get_total_cycles(&self) -> u64 {
        self.total_cycles
    }

    pub fn get_pc(&self) -> u32 {
        self.registers.get_pc()
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.registers.set_pc(pc)
    }

    pub fn is_in_thumb_mode(&self) -> bool {
        self.registers.is_thumb()
    }

    pub fn get_cpsr(&self) -> u32 {
        self.registers.get_cpsr()
    }

    pub fn switch_to_thumb(&mut self) {
        self.registers.set_thumb_state(true);
    }

    pub fn switch_to_arm(&mut self) {
        self.registers.set_thumb_state(false);
    }

    pub fn get_registers(&self) -> &registers::Registers {
        &self.registers
    }

    pub fn set_debug_mode(enabled: bool) {
        DEBUG_PRINT.store(enabled, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn reset_debug_count() {
        DEBUG_COUNT.store(0, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn store_prefetch(&mut self) {}
}
