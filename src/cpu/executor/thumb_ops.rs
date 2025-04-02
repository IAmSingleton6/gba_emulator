use crate::cpu::CPU;

pub type ThumbExecutor = fn(&mut CPU, u16) -> i32;
