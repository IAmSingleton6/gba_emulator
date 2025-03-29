use crate::gb::cpu::CPU;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub name: &'static str,
    pub cycles: u8,
    pub size: u8,
    pub flags: &'static [crate::gb::registers::FlagBits],
    pub execute: fn(&Instruction, &mut CPU) -> u64, // Return number on M-Cycles needed to execute
}

// const fn create_opcodes() -> [Option<&'static Instruction>; 256] {
//     // This will create table of main instructions
// }

// const fn create_cb_opcodes() -> [Option<&'static Instruction>; 256] {
//     // This will create table of "CB" subset instructions
// }

// // Declaring constant, public, always accessible instructions tables
// pub const OPCODES: [Option<&'static Instruction>; 256] = create_opcodes();
// pub const OPCODES_CB: [Option<&'static Instruction>; 256] = create_cb_opcodes();