/*
    R 0->12: GPR
    R 13: Stack Pointer
    R 14: Link Register
    R15: PC
*/
use num_enum::TryFromPrimitive;
use super::operations::{ArithResult, CARRY_BIT, SIGN_BIT};

pub struct Registers {
    pub r: [u32; 16],
    pub cpsr: u32
}

#[repr(u32)]
enum CpsrMasks {
    SignFlag     = 0x8000_0000,
    ZeroFlag     = 0x4000_0000,
    CarryFlag    = 0x2000_0000,
    OverflowFlag = 0x1000_0000,
    IrqDisable   = 0x0000_0080,
    FiqDisable   = 0x0000_0040,
    ThumbMode    = 0x0000_0020,
    CpuMode      = 0x0000_001F,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
enum CpuMode {
    User   = 0x10,
    Fiq    = 0x11,
    Irq    = 0x12,
    Svc    = 0x13,
    Abort  = 0x17,
    Undef  = 0x1B,
    System = 0x1F,
    Undefined
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            cpsr: 0
        }
    }

    pub fn get_pc(&self) -> u32 { self.r[15] }
    pub fn set_pc(&mut self, value: u32) { self.r[15] = value }
    pub fn get_sign(&self) -> u32 { (self.cpsr & CpsrMasks::SignFlag as u32) >> 31 }
    pub fn set_sign(&mut self, value: bool) { if value { self.cpsr |= (CpsrMasks::SignFlag as u32) } else { self.cpsr &= !(CpsrMasks::SignFlag as u32) } }
    pub fn get_zero(&self) -> u32 { (self.cpsr & CpsrMasks::ZeroFlag as u32) >> 30 }
    pub fn set_zero(&mut self, value: bool) { if value { self.cpsr |= (CpsrMasks::ZeroFlag as u32) } else { self.cpsr &= !(CpsrMasks::ZeroFlag as u32) } }
    pub fn get_carry(&self) -> u32 { (self.cpsr & CpsrMasks::CarryFlag as u32) >> 29 }
    pub fn set_carry(&mut self, value: bool) { if value { self.cpsr |= (CpsrMasks::CarryFlag as u32) } else { self.cpsr &= !(CpsrMasks::CarryFlag as u32) } }
    pub fn get_overflow(&self) -> u32 { (self.cpsr & CpsrMasks::OverflowFlag as u32) >> 28 }
    pub fn set_overflow(&mut self, value: bool) { if value { self.cpsr |= (CpsrMasks::OverflowFlag as u32) } else { self.cpsr &= !(CpsrMasks::OverflowFlag as u32) } }

    pub fn get_current_cpu_mode(&self) -> CpuMode { CpuMode::try_from(self.cpsr & CpsrMasks::CpuMode as u32).unwrap_or(CpuMode::Undefined) }
    pub fn has_spsr(&self) -> bool { self.get_current_cpu_mode() != CpuMode::User && self.get_current_cpu_mode() != CpuMode::System }

    pub fn conditional_set_all_flags(&mut self, set_flags: bool, result: ArithResult) {
        if set_flags {
            self.set_all_flags(result);
        }
    }
    
    fn set_all_flags(&mut self, result: ArithResult) {
        self.set_sign((result.value & SIGN_BIT) != 0);
        self.set_zero(result.value as u32 == 0);
        self.set_carry((result.value & CARRY_BIT) != 0);
        self.set_overflow(result.overflow);
    }
}