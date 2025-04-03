pub const CARRY_BIT: u64 = 0x1_0000_0000;
pub const SIGN_BIT: u64 = 0x8000_0000;

pub enum Condition {
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

pub enum ShiftType {
    LSL = 0,
    LSR = 1,
    ASR = 2,
    ROR = 3,
    RRX = 4
}

pub struct ResultWithCarry {
    result: u32,
    carry: u32
}

pub struct ArithResult {
    pub value: u64,
    pub overflow: bool
}

pub type ArithOp = fn(u32, u32, u32) -> ArithResult;
pub type MullOp = fn(u32, u32, u32, u32) -> i64;
pub type LogicOp = fn(u32, u32) -> u32;
// type LoadOp = fn(&memory, u32) -> (u32, i32);
// type StoreOp = fn(&memory, u32, u32) -> i32;