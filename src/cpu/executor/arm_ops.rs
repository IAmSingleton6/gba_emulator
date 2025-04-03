use crate::cpu::CPU; 
use super::operations::{ArithResult, Condition};

pub type ArmExecutor = fn(&mut CPU, u32) -> i32;

impl CPU {
    pub fn arm_no_op(&mut self, _opcode: u32) -> i32 {
        0
    }
    
    pub fn arm_branch_and_branch_exchange(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_block_data_transfer(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_branch(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_software_interrupt(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_undefined(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_single_data_transfer(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_single_data_swap(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_multiply(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_multiply_long(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_halfword_data_transfer_register(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_halfword_data_transfer_immediate(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_psr_transfer_mrs(&mut self, opcode: u32) -> i32 {

    }
    
    pub fn arm_psr_transfer_msr(&mut self, opcode: u32) -> i32 {

    }
    
    pub fn arm_data_processing(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_adc_imm(&mut self, condition: Condition, set_flags: bool, n: Reg, d: Reg, imm: u32) -> i32 {
        arm_arith_imm(set_flags, n, d, imm, add_op, self.registers.get_carry())
    }

    fn arm_arith_imm(&mut self, set_flags: bool, n: Reg, d: Reg, imm: u32, op: ArithOp, carry: u32) -> i32 {
        imm = arm_expand_immediate(imm);
        let result: ArithResult = op(registers.r[n], imm, carry);
    
        if d == self.registers.get_pc() {
            return alu_write_pc(set_flags, result.value);
        } else {
            self.registers.r[d] = result.value;
            self.registers.conditional_set_all_flags(set_flags, result);
        }
    
        return 0;
    }

    fn alu_write_pc(&mut self, set_flags: bool, result: u32) -> i32 {
        if set_flags && self.registers.has_spsr() {
            return return_from_exception(result);
        } else {
            return arm_branch_write_pc(result);
        }
    }
}