/*
    R 0->12: GPR
    R 13: Stack Pointer
    R 14: Link Register
    R15: PC
*/


pub struct Registers {
    pub r: [u32; 16],
    pub cpsr: u32
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            cpsr: 0
        }
    }

    pub fn get_pc(&self) -> u32 {
        self.r[15]
    }

    pub fn set_pc(&mut self, value: u32) {
        self.r[15] = value
    }
}