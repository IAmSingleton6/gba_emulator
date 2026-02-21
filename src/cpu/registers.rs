/*
    R 0->12: GPR
    R 13: Stack Pointer
    R 14: Link Register
    R15: PC
*/
use num_enum::TryFromPrimitive;

pub struct Registers {
    r: [u32; 16],
    cpsr: u32
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

impl CpsrMasks {
    fn bits(self) -> u32 { self as u32 }
}

pub trait FlagResult {
    fn apply_flags(&self, regs: &mut Registers);
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

    // Registers
    pub fn get_r(&self, index: usize) -> u32 { debug_assert!(index < 16); self.r[index] }
    pub fn set_r(&mut self, index: usize, value: u32) { debug_assert!(index < 16); self.r[index] = value }
    pub fn get_sp(&self) -> u32 { self.r[13] }
    pub fn set_sp(&mut self, value: u32) { self.r[13] = value }
    pub fn get_lr(&self) -> u32 { self.r[14] }
    pub fn set_lr(&mut self, value: u32) { self.r[14] = value }
    pub fn get_pc(&self) -> u32 { self.r[15] }
    pub fn get_visible_pc(&self) -> u32 { self.get_pc().wrapping_add(if self.is_thumb() { 4 } else { 8 }) }
    pub fn set_pc(&mut self, value: u32) { self.r[15] = value }

    // CPSR
    pub fn get_cpsr(&self) -> u32 { self.cpsr }
    pub fn set_cpsr(&mut self, value: u32) { self.cpsr = value }
    pub fn get_sign(&self) -> bool { (self.cpsr & CpsrMasks::SignFlag.bits()) != 0 }
    pub fn set_sign(&mut self, value: bool) { if value { self.cpsr |= CpsrMasks::SignFlag.bits() } else { self.cpsr &= !CpsrMasks::SignFlag.bits() } }
    pub fn get_zero(&self) -> bool { (self.cpsr & CpsrMasks::ZeroFlag.bits()) != 0 }
    pub fn set_zero(&mut self, value: bool) { if value { self.cpsr |= CpsrMasks::ZeroFlag.bits() } else { self.cpsr &= !CpsrMasks::ZeroFlag.bits() } }
    pub fn get_carry(&self) -> bool { (self.cpsr & CpsrMasks::CarryFlag.bits()) != 0 }
    pub fn set_carry(&mut self, value: bool) { if value { self.cpsr |= CpsrMasks::CarryFlag.bits() } else { self.cpsr &= !CpsrMasks::CarryFlag.bits() } }
    pub fn get_overflow(&self) -> bool { (self.cpsr & CpsrMasks::OverflowFlag.bits()) != 0 }
    pub fn set_overflow(&mut self, value: bool) { if value { self.cpsr |= CpsrMasks::OverflowFlag.bits() } else { self.cpsr &= !CpsrMasks::OverflowFlag.bits() } }

    pub fn get_current_cpu_mode(&self) -> CpuMode { CpuMode::try_from(self.cpsr & CpsrMasks::CpuMode.bits()).unwrap_or(CpuMode::Undefined) }
    pub fn has_spsr(&self) -> bool { self.get_current_cpu_mode() != CpuMode::User && self.get_current_cpu_mode() != CpuMode::System }

    pub fn is_thumb(&self) -> bool { (self.cpsr & CpsrMasks::ThumbMode.bits()) != 0 }
    pub fn set_thumb_state(&mut self, enabled: bool) {
        if enabled {
            self.cpsr |= CpsrMasks::ThumbMode.bits();
        } else {
            self.cpsr &= !(CpsrMasks::ThumbMode.bits());
        }
    }

    pub fn mov(&mut self, rd: usize, nn: u32) {
        self.set_r(rd, nn);
        self.set_flags(&nn);
    }

    pub fn set_flags<T: FlagResult>(&mut self, result: &T) {
        result.apply_flags(self);
    }

    pub fn set_flags_if<T: FlagResult>(&mut self, condition: bool, result: &T) {
        if condition {
            self.set_flags(result);
        }
    }
}