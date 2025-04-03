use decoder::{ decode_arm, decode_thumb };
use executor::{ ArmExecutor, ThumbExecutor };

mod registers;
mod decoder;
mod operations;
pub mod executor;


pub struct CPU {
    registers: registers::Registers
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: registers::Registers::new()
        }
    }

    pub fn fetch_decode_execute(&mut self, memory: &mut [u32]) {
        let is_in_thumb_mode: bool = self.is_in_thumb_mode();
        let opcode: u32 = self.fetch(memory, is_in_thumb_mode);
        if is_in_thumb_mode {
            let executor: ThumbExecutor = decode_thumb(opcode as u16);
            let cycles: i32 = executor(self, opcode as u16);
        } else {
            let executor: ArmExecutor = decode_arm(opcode);
            let cycles: i32 = executor(self, opcode);
        }
    }

    pub fn fetch(&mut self, memory: &mut [u32], is_in_thumb_mode: bool) -> u32 {
        let pc: u32 = self.registers.get_pc();
        let instruction: u32 = memory[pc as usize / 4];
        self.registers.set_pc(pc.wrapping_add(if is_in_thumb_mode {2} else {4}));
        instruction
    }

    fn is_in_thumb_mode(&self) -> bool {
        (self.registers.cpsr & 0x20) != 0
    }

    pub fn switch_to_thumb(&mut self) {
        self.registers.cpsr |= 0x20; // Set the T-bit to 1
    }

    pub fn switch_to_arm(&mut self) {
        self.registers.cpsr &= !0x20; // Clear the T-bit (set to 0)
    }
}



