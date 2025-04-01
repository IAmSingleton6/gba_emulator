use super::CPU;
use super::ArithResult;

impl CPU {
    // Returns the number of cycles taken to execute the instruction
    pub fn execute_arm_instruction(&mut self, opcode: u32) -> i32 {

    }

    pub fn arm_adc_imm(condition: Condition, set_flags: bool, n: Reg, d: Reg, imm: u32) -> i32 {
        arm_arith_imm(set_flags, n, d, imm, add_op, registers.get_carry())
    }

    fn arm_arith_imm(set_flags: bool, n: Reg, d: Reg, imm: u32, op: ArithOp, carry: u32) -> i32 {
        imm = arm_expand_immediate(imm);
        let result: ArithResult = op(registers.r[n], imm, carry);
    
        if d == pc {
            return alu_write_pc(set_flags, result.value);
        } else {
            registers.r[d] = result.value;
            registers.conditional_set_all_flags(set_flags, result);
        }
    
        return 0;
    }

    fn alu_write_pc(set_flags: bool, result: u32) -> i32 {
        if set_flags && has_spsr() {
            return return_from_exception(result);
        } else {
            return arm_branch_write_pc(result);
        }
    }
}

