use crate::cpu::CPU; 
use crate::cpu::operations::{Condition, add_op, and_op, asr_op, eor_op, lsl_op, lsr_op, mul_op, mvn_op, orr_op, ror_op, should_branch, sub_op};
use simple_bits::BitsExt;

pub type ThumbExecutor = fn(&mut CPU, u16) -> u64;

impl CPU {
    pub fn thumb_no_op(&mut self, _opcode: u16) -> u64 {
        0
    }
    
    // COMPLETE
    pub fn thumb_move_shifted_register(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(11..13);
        let offset = opcode.extract_bits(6..11) as u32;
        let rs = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        let value = self.registers.get_r(rs);
        let old_carry = self.registers.get_carry();

        let shift_op = match op {
            0b00 => lsl_op,
            0b01 => lsr_op,
            0b10 => asr_op,
            _ => unreachable!(),
        };

        let result = shift_op(value, offset, old_carry);
        self.registers.set_r(rd, result.result());
        self.registers.set_flags(&result);

        // 1S 
        1 
    }

    // COMPLETE
    pub fn thumb_add_subtract(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(9..11);
        let nn = opcode.extract_bits(6..9);
        let rs = opcode.extract_bits(3..6);
        let rd = opcode.extract_bits(0..3);

        let rs_val = self.registers.get_r(rs as usize);

        let operand = if op < 2 {
            self.registers.get_r(nn as usize)
        } else {
            nn as u32
        };

        let result = match op {
            0 => add_op(rs_val, operand, self.registers.get_carry()),  // ADD {S} Rd, Rs, Rn
            1 => sub_op(rs_val, operand, self.registers.get_carry()),  // SUB {S} Rd, Rs, Rn
            2 => add_op(rs_val, operand, self.registers.get_carry()),  // ADD {S} Rd, Rs, #nn
            3 => sub_op(rs_val, operand, self.registers.get_carry()),  // SUB {S} Rd, Rs, #nn
            _ => panic!("Invalid add subtract operation"),
        };

        self.registers.set_r(rd as usize, result.result());
        self.registers.set_flags(&result);

        // 1S
        1
    }

    // COMPLETE
    pub fn thumb_move_compare_add_subtract_immediate(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(11..13);
        let rd = opcode.extract_bits(8..11) as usize;
        let nn = opcode.extract_bits(0..8) as u32;

        let cycles = 1 as u64;
        let rs_val = self.registers.get_r(rd as usize);

        if op == 0 {
            self.registers.mov(rd, nn);
            return cycles
        }

        let arith_op = match op {
            1 => { sub_op }
            2 => { add_op }
            3 => { sub_op }
            _ => panic!("Invalid operation in thumb_move_compare_add_sub_immediate"),
        };

        let result = arith_op(rs_val, nn, self.registers.get_carry());
        self.registers.set_flags(&result);
        if op != 1 {
            self.registers.set_r(rd, result.result());
        }

        // 1S
        cycles
    }

    pub fn thumb_alu_operations(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(6..10);
        let rs = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        let rs_val = self.registers.get_r(rs);
        let rd_val = self.registers.get_r(rd);

        /*
        N,Z,C,V for  ADC,SBC,NEG,CMP,CMN
        N,Z,C   for  LSL,LSR,ASR,ROR (carry flag unchanged if zero shift amount)
        N,Z,C   for  MUL on ARMv4 and below: carry flag destroyed
        N,Z     for  MUL on ARMv5 and above: carry flag unchanged
        N,Z     for  AND,EOR,TST,ORR,BIC,MVN
         */

        let cycles = match op {
            0x0 => { // AND{S}
                let result = and_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0x1 => { // EOR{S}
                let result = eor_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0x2 => { // LSL{S}
                let result = lsl_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x3 => { // LSR{S}
                let result = lsr_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x4 => { // ASR{S}
                let result = asr_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x5 => { // ADC{S}
                // TODO: Check whether all others should add with carry as they currently do
                let result = add_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            0x6 => { // SBC{S}
                // TODO: Check whether all others should sub with carry as they currently do
                let result = sub_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            0x7 => { // ROR{S}
                let result = ror_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x8 => { // TST
                let result = and_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                1
            }
            0x9 => { // NEG{S}
                let result = sub_op(0, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            0xA => { // CMP
                let result = sub_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                1
            }
            0xB => { // CMN
                let result = add_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                1
            }
            0xC => { // ORR{S}
                let result = mvn_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0xD => { // MUL{S}
                let result = mul_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0xE => { // BIC{S}
                let result = and_op(rd_val, !rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0xF => { // MVN{S}
                let result = mvn_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
        }

        return cycles
    }

    pub fn thumb_hi_register_operations_branch_exchange(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(8..10);
        let msb_d = opcode.extract_bit(7);
        let msb_s = opcode.extract_bit(6);
        let rs = opcode.extract_bits(3..6);
        let rd = opcode.extract_bits(0..3);
    }

    pub fn thumb_pc_relative_load(&mut self, opcode: u16) -> u64 {
        let rd = opcode.extract_bits(8..11);
        let nn = opcode.extract_bits(0..8);
    }
    
    pub fn thumb_load_store_with_register_offset(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(10..12);
        let ro = opcode.extract_bits(6..9);
        let rb = opcode.extract_bits(3..6);
        let rd = opcode.extract_bits(0..3);
    }
    
    pub fn thumb_load_store_sign_extended_byte_halfword(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(10..12);
        let ro = opcode.extract_bits(6..9);
        let rb = opcode.extract_bits(3..6);
        let rd = opcode.extract_bits(0..3);
    }

    pub fn thumb_load_store_with_immediate_offset(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(11..13);
        let nn = opcode.extract_bits(6..9);
        let rb = opcode.extract_bits(3..6);
        let rd = opcode.extract_bits(0..3);
    }

    pub fn thumb_load_store_halfword(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let nn = opcode.extract_bits(6..11);
        let rb = opcode.extract_bits(3..6);
        let rd = opcode.extract_bits(0..3);
    }
    
    pub fn thumb_sp_relative_load_store(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let rd = opcode.extract_bits(8..11);
        let nn = opcode.extract_bits(0..8);
    }

    pub fn thumb_get_relative_address(&mut self, opcode: u16) -> u64 {
        
    }

    pub fn thumb_add_offset_to_stack_pointer(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(7);
        let nn = opcode.extract_bits(0..7);
    }

    pub fn thumb_push_pop_registers(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let pc_lr = opcode.extract_bit(8);
        let r_list = opcode.extract_bits(0..8) as u8;

        if op {
            thumb_pop(pc_lr, r_list)
        } else {
            thumb_push(pc_lr, r_list)
        }
    }

    pub fn thumb_multiple_load_store(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let rb = opcode.extract_bits(8..11);
        let r_list = opcode.extract_bits(0..8);
    }

    // COMPLETE
    pub fn thumb_unconditional_branch(&mut self, opcode: u16) -> u64 {
        let signed_offset_11 = opcode.extract_bits(0..11);   
        let signed_offset = ((signed_offset_11 << 21) >> 21) as i32;
        // 2 bytes - halfword aligned
        let branch_offset = signed_offset << 1;

        let pc = self.registers.get_visible_pc();
        let new_pc = pc.wrapping_add_signed(branch_offset) as u32;

        self.registers.set_pc(new_pc);

        // 2S + 1N 
        3
    }

    // COMPLETE
    pub fn thumb_conditional_branch(&mut self, opcode: u16) -> u64 {
        let signed_offset_8 = opcode.extract_bits(0..8);
        let signed_offset = ((signed_offset_8 << 24) >> 24) as i32;
        // Halfword aligned
        let branch_offset = signed_offset << 1;

        let op = opcode.extract_bits(8..12);
        let pc = self.registers.get_visible_pc();
        let mut cycles = 1 as u64;

        if should_branch(&self.registers, Condition::from(op)) {
            cycles += 2;
            let new_pc = pc.wrapping_add_signed(branch_offset);
            self.registers.set_pc(new_pc);
        } 

        // 2S + 1N OR
        // 1S
        cycles
    }

    // COMPLETE
    pub fn thumb_long_branch_with_link_1(&mut self, opcode: u16) -> u64 {
        let upper_bits = opcode.extract_bits(0..11);  
        let shifted_upper = (upper_bits as i32) << 12;  

        let pc = self.registers.get_visible_pc();
        let lr = pc.wrapping_add_signed(shifted_upper);
        self.registers.set_lr(lr);

        // 1S
        1
    }

    // COMPLETE
    pub fn thumb_long_branch_with_link_2(&mut self, opcode: u16) -> u64 {
        let lower_bits = opcode.extract_bits(0..11); 
        let shifted_lower = (lower_bits as i32) << 1;

        let lr = self.registers.get_lr() as i32;
        let pc = self.registers.get_pc();
        let new_pc = (lr.wrapping_add(shifted_lower) as u32) & !1; // Ensure halfword alignment

        self.registers.set_pc(new_pc);

        let new_lr = pc.wrapping_add(2) as u32; 
        self.registers.set_lr(new_lr | 1);

        // BLX
        if ! opcode.extract_bit(12) { 
            self.registers.set_thumb_state(false);
        }

        // 2S + 1N
        3
    }

    pub fn thumb_software_interrupt(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(8..16);
        let nn = opcode.extract_bits(0..8);
    }

    pub fn thumb_breakpoint(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(8..16);
        let nn = opcode.extract_bits(0..8);
    }

    fn thumb_push(&mut self, lr: bool, r_list: u8) -> u64 {
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

    fn thumb_pop(&mut self, pc: bool, r_list: u8) -> u64 {

    }
}