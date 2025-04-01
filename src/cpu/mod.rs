use decoder::{decode_arm, decode_thumb, ArmInstruction, ThumbInstruction};

mod registers;
mod operations;
mod decoder;
mod arm_ops;

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
            let instruction: ThumbInstruction = decode_thumb(opcode as u16);
            let cycles: i32 = execute_thumb_instruction(instruction);
        } else {
            let instruction: ArmInstruction = decode_arm(opcode);
            let cycles: i32 = execute_arm_instruction(instruction);
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

enum Condition {
    Equal         = 0b0000,
    NotEqual      = 0b0001,
    CarrySet      = 0b0010,
    CarryClear    = 0b0011,
    Minus         = 0b0100,
    Plus          = 0b0101,
    OverflowSet   = 0b0110,
    OverflowClear = 0b0111,
    Higher        = 0b1000,
    LowerSame     = 0b1001,
    GreaterEqual  = 0b1010,
    LessThan      = 0b1011,
    GreaterThan   = 0b1100,
    LessEqual     = 0b1101,
    Always        = 0b1110,
}

enum ShiftType {
    LSL = 0,
    LSR = 1,
    ASR = 2,
    ROR = 3,
    RRX = 4
}

