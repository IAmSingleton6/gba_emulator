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

pub const CARRY_BIT: u64 = 0x1_0000_0000;
pub const SIGN_BIT: u64 = 0x8000_0000;