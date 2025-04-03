use crate::cpu::CPU; 
use crate::cpu::operations::{ArithResult, Condition};
use simple_bits::BitsExt;

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
        let op = opcode.extract_bit(11);
        let pc_lr = opcode.extract_bit(8);
        let r_list = opcode.extract_bits(0..7) as u8;

        if op {
            thumb_pop(pc_lr, r_list)
        } else {
            thumb_push(pc_lr, r_list)
        }
    }

    fn thumb_push(&mut self, lr: bool, r_list: u8) -> i32 {
        let sp = self.registers.get_sp();
        sp = sp - 4 * r_list.count_ones(); 
        // Why?
        if lr {
            sp = sp - 4;
        }
        self.registers.set_sp(sp);
        let addr = sp;

        for i in 0..8 {
            if (r_list & (1 << i)) != 0 {
                self.memory.write(addr, self.registers.get_r(i));
                cycles += self.memory.access_time<u32>(addr);
                // Why?
                addr += 4;
            }
        }

        if lr {
            self.memory.write(addr, self.registers.get_lr());
            cycles += self.memory.access_time<u32>(addr);
        }
        // ???????
        store_prefetch();

        cycles
    }

    fn thumb_pop(&mut self, pc: bool, r_list: u8) -> i32 {

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
        let op = opcode.extract_bits(10..11);
        let ro = opcode.extract_bits(6..8);
        let rb = opcode.extract_bits(3..5);
        let rd = opcode.extract_bits(0..2);


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