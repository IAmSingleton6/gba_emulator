use num_enum::FromPrimitive;

use crate::cpu::registers::{FlagResult, Registers};

pub const CARRY_BIT: u32 = 0x1_0000_0000;
pub const SIGN_BIT: u32 = 0x8000_0000;

#[repr(u16)]
#[derive(Debug, FromPrimitive)]
pub enum Condition {
    Equal = 0x0,
    NotEqual = 0x1,
    CarrySet = 0x2,
    CarryClear = 0x3,
    Minus = 0x4,
    Plus = 0x5,
    OverflowSet = 0x6,
    OverflowClear = 0x7,
    Higher = 0x8,
    LowerSame = 0x9,
    GreaterEqual = 0xA,
    LessThan = 0xB,
    GreaterThan = 0xC,
    LessEqual = 0xD,
    Always = 0xE,
    #[num_enum(catch_all)] 
    Invalid(u16),
}

pub fn should_branch(registers: &Registers, condition: Condition) -> bool {
    match condition {
        Condition::Equal => registers.get_zero(),
        Condition::NotEqual => !registers.get_zero(),
        Condition::CarrySet => registers.get_carry(),
        Condition::CarryClear => !registers.get_carry(),
        Condition::Minus => registers.get_sign(),
        Condition::Plus => !registers.get_sign(),
        Condition::OverflowSet => registers.get_overflow(),
        Condition::OverflowClear => !registers.get_overflow(),
        Condition::Higher => registers.get_carry() && !registers.get_zero(),
        Condition::LowerSame => !registers.get_carry() || registers.get_zero(),
        Condition::GreaterEqual => registers.get_sign() == registers.get_overflow(),
        Condition::LessThan => registers.get_sign() != registers.get_overflow(),
        Condition::GreaterThan => !registers.get_zero() && registers.get_sign() == registers.get_overflow(),
        Condition::LessEqual => registers.get_zero() || registers.get_sign() != registers.get_overflow(),
        Condition::Always => true,
        Condition::Invalid(_) => panic!("Encountered invalid conditional"),
    }
}

pub type LogicOp = fn(u32, u32) -> u32;
pub const fn and_op(v1: u32, v2: u32) -> u32 {
    v1 & v2
}
pub const fn bic_op(v1: u32, v2: u32) -> u32 {
    v1 & !v2
}
pub const fn eor_op(v1: u32, v2: u32) -> u32 {
    v1 ^ v2
}
pub const fn orr_op(v1: u32, v2: u32) -> u32 {
    v1 | v2
}
pub const fn mvn_op(_: u32, v2: u32) -> u32 {
    !v2
}

pub type MulOp = fn(u32, u32, u32, u32) -> i64;
pub const fn mul_op() -> ArithResult {

}

pub type ArithOp = fn(u32, u32, bool) -> ArithResult;
pub const fn add_op(v1: u32, v2: u32, carry: bool) -> ArithResult {
    add_with_carry(v1, v2, carry)
}
pub const fn sub_op(v1: u32, v2: u32, carry: bool) -> ArithResult {
    add_with_carry(v1, !v2, carry)
}
pub const fn rsb_op(v1: u32, v2: u32, carry: bool) -> ArithResult {
    add_with_carry(!v1, v2, carry)
}

pub const fn add_with_carry(v1: u32, v2: u32, carry: bool) -> ArithResult {
    let carry_in = if carry { 1 } else { 0 };

    let (r1, c1) = v1.overflowing_add(v2);
    let (result, c2) = r1.overflowing_add(carry_in);

    let carry_out = c1 || c2;

    let overflow = ((v1 ^ result) & (v2 ^ result) & 0x8000_0000) != 0;

    ArithResult {
        result,
        carry: carry_out,
        overflow,
    }
}

pub struct ArithResult {
    result: u32,
    carry: bool,
    overflow: bool,
}

impl ArithResult {
    pub fn result(&self) -> u32 {
        self.result
    }

    pub fn carry(&self) -> bool {
        self.carry
    }

    pub fn overflow(&self) -> bool {
        self.overflow
    }
}

pub struct ResultWithCarry {
    result: u32,
    carry: bool,
}

impl ResultWithCarry {
    pub fn result(&self) -> u32 {
        self.result
    }

    pub fn carry(&self) -> bool {
        self.carry
    }
}

impl FlagResult for ArithResult {
    fn apply_flags(&self, regs: &mut Registers) {
        regs.set_sign((self.result() & SIGN_BIT) != 0);
        regs.set_zero(self.result() == 0);
        regs.set_carry(self.carry());
        regs.set_overflow(self.overflow());
    }
}

impl FlagResult for ResultWithCarry {
    fn apply_flags(&self, regs: &mut Registers) {
        regs.set_sign((self.result() & SIGN_BIT) != 0);
        regs.set_zero(self.result() == 0);
        regs.set_carry(self.carry());
    }
}

impl FlagResult for u32 {
    fn apply_flags(&self, regs: &mut Registers) {
        regs.set_sign((*self & SIGN_BIT) != 0);
        regs.set_zero(*self == 0);
    }
}

pub type ShiftOp = fn(u32, u32, bool) -> ResultWithCarry;

pub const fn lsl_op(value: u32, amount: u32, carry: bool) -> ResultWithCarry {
    match amount {
        0 => ResultWithCarry {
            result: value,
            carry: carry,
        },

        1..=31 => ResultWithCarry {
            result: value << amount,
            carry: (value >> (32 - amount)) & 1 != 0,
        },

        32 => ResultWithCarry {
            result: 0,
            carry: (value & 1) != 0,
        },

        _ => ResultWithCarry {
            result: 0,
            carry: false,
        },
    }
}

pub const fn lsr_op(value: u32, amount: u32, _carry: bool) -> ResultWithCarry {
    let shift = if amount == 0 { 32 } else { amount };

    match shift {
        1..=31 => ResultWithCarry {
            result: value >> shift,
            carry: (value >> (shift - 1)) & 1 != 0,
        },

        _ => ResultWithCarry {
            result: 0,
            carry: (value >> 31) & 1 != 0,
        },
    }
}

pub const fn asr_op(value: u32, amount: u32, _carry: bool) -> ResultWithCarry {
    let shift = if amount == 0 { 32 } else { amount };

    match shift {
        1..=31 => ResultWithCarry {
            result: ((value as i32) >> shift) as u32,
            carry: (value >> (shift - 1)) & 1 != 0,
        },

        _ => {
            let sign = (value >> 31) & 1 != 0;

            ResultWithCarry {
                result: if sign { u32::MAX } else { 0 },
                carry: sign,
            }
        }
    }
}

pub const fn ror_op(value: u32, amount: u32, carry: bool) -> ResultWithCarry {
    let rot = amount & 31;

    if rot == 0 {
        // IMPORTANT:
        // For ARM register shifts:
        // ROR #0 means RRX (not plain no-op)
        let carry_in = if carry { 1 } else { 0 };

        ResultWithCarry {
            result: (carry_in << 31) | (value >> 1),
            carry: (value & 1) != 0,
        }
    } else {
        let result = value.rotate_right(rot);

        ResultWithCarry {
            result,
            carry: (result >> 31) & 1 != 0,
        }
    }
}

pub const fn rrx_op(value: u32, _amount: u32, carry: bool) -> ResultWithCarry {
    let carry_in = if carry { 1 } else { 0 };

    ResultWithCarry {
        result: (carry_in << 31) | (value >> 1),
        carry: (value & 1) != 0,
    }
}