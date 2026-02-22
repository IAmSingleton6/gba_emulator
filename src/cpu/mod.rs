use decoder::{decode_arm, decode_thumb};
use executor::{ArmExecutor, ThumbExecutor};

use crate::{cpu::registers::Registers, memory::MemoryAccess};

mod decoder;
pub mod executor;
mod operations;
mod registers;

pub struct CPU {
    registers: registers::Registers,
    memory: Box<dyn MemoryAccess>,
    total_cycles: u64
}

impl CPU {
    pub fn new(memory: Box<dyn MemoryAccess>) -> Self {
        CPU {
            registers: Registers::new(),
            memory,
            total_cycles: 0,
        }
    }

    pub fn fetch_decode_execute(&mut self) {
        let is_in_thumb_mode: bool = self.is_in_thumb_mode();
        let opcode: u32 = self.fetch(is_in_thumb_mode);

        let cycles= if is_in_thumb_mode {
            let executor: ThumbExecutor = decode_thumb(opcode as u16);
            executor(self, opcode as u16)
        } else {
            let executor: ArmExecutor = decode_arm(opcode);
            executor(self, opcode)
        };

        self.total_cycles += cycles;
    }

    pub fn fetch(&mut self, is_in_thumb_mode: bool) -> u32 {
        let pc: u32 = self.registers.get_pc();
        let instruction = self.memory.read_u32(pc);
        self.registers.set_pc(pc.wrapping_add(if is_in_thumb_mode { 2 } else { 4 }));
        instruction
    }

    pub fn is_in_thumb_mode(&self) -> bool {
        self.registers.is_thumb()
    }

    fn switch_to_thumb(&mut self) {
        self.registers.set_thumb_state(true);
    }

    fn switch_to_arm(&mut self) {
        self.registers.set_thumb_state(false);
    }

    fn read_memory(&self, address: u32) -> u32 {
        self.memory.read_u32(address)  
    }

    fn write_memory(&mut self, address: u32, value: u32) {
        self.memory.write_u32(address, value)  
    }
}
