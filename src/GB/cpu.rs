use crate::gb::registers;
use crate::gb::instructions;

pub struct CPU {
    pub registers: registers::Registers,
    pub ime: bool,      // Interrupt Master Enable - True if you want to enable and intercept interrupts
    pub opcode: u8,     // Running Instruction Opcode
    pub cycles: u64,    // Total Cycles Count
}

impl CPU {
    pub fn new() -> Self {
        CPU { 
            registers: registers::Registers::new(),
            ime: false,
            opcode: 0,
            cycles: 0,
         }
    }

    pub fn fetch_next(&mut self) -> u8 {
        1
    }

    pub fn decode(opcode: u8, cb_opcode: bool) -> Option<&'static instructions::Instruction> {
        None
    }

    pub fn execute_next(&mut self) -> u64 {
        1
    }
}