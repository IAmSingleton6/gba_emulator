use decoder::{decode_arm, decode_thumb};
use executor::{ArmExecutor, ThumbExecutor};

mod decoder;
pub mod executor;
mod operations;
mod registers;

pub struct CPU {
    registers: registers::Registers,
    total_cycles: u64
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers::new(),
            total_cycles: 0,
        }
    }

    pub fn fetch_decode_execute(&mut self, memory: &mut [u32]) {
        let is_in_thumb_mode: bool = self.is_in_thumb_mode();
        let opcode: u32 = self.fetch(memory, is_in_thumb_mode);

        let cycles= if is_in_thumb_mode {
            let executor: ThumbExecutor = decode_thumb(opcode as u16);
            executor(self, opcode as u16)
        } else {
            let executor: ArmExecutor = decode_arm(opcode);
            executor(self, opcode)
        };

        self.total_cycles += cycles as u64;
    }

    pub fn fetch(&mut self, memory: &mut [u32], is_in_thumb_mode: bool) -> u32 {
        let pc: u32 = self.registers.get_pc();
        let instruction: u32 = memory[pc as usize / 4];
        self.registers.set_pc(pc.wrapping_add(if is_in_thumb_mode { 2 } else { 4 }));
        instruction
    }

    fn is_in_thumb_mode(&self) -> bool {
        self.registers.is_thumb()
    }

    pub fn switch_to_thumb(&mut self) {
        self.registers.set_thumb_state(true);
    }

    pub fn switch_to_arm(&mut self) {
        self.registers.set_thumb_state(false);
    }
}
