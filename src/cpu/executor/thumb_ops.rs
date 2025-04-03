use crate::cpu::CPU; 
use crate::cpu::operations::{ArithResult, Condition};

pub type ThumbExecutor = fn(&mut CPU, u16) -> i32;

impl CPU {
    pub fn thumb_no_op(&mut self, _opcode: u16) -> i32 {
        0
    }

    pub fn thumb_software_interrupt(&mut self, opcode: u16) -> i32 {

    }

    pub fn thumb_unconditional_branch(&mut self, opcode: u16) -> i32 {

    }

    pub fn thumb_conditional_branch(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_multiple_load_store(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_long_branch_with_link(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_add_offset_to_stack_pointer(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_push_pop_registers(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_load_store_halfword(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_sp_relative_load_store(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_load_address(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_load_store_with_immediate_offset(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_load_store_with_register_offset(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_load_store_sign_extended_byte_halfword(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_pc_relative_load(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_hi_register_operations_branch_exchange(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_alu_operations(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_move_compare_add_subtract_immediate(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_add_subtract(&mut self, opcode: u16) -> i32 {

    }
    
    pub fn thumb_move_shifted_register(&mut self, opcode: u16) -> i32 {

    }
}