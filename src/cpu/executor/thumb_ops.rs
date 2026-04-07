use crate::cpu::operations::{
    add_op, and_op, asr_op, eor_op, lsl_op, lsr_op, mul_op, mvn_op, ror_op, should_branch, sub_op,
    Condition,
};
use crate::cpu::CPU;
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
            0 => add_op(rs_val, operand, self.registers.get_carry()), // ADD {S} Rd, Rs, Rn
            1 => sub_op(rs_val, operand, self.registers.get_carry()), // SUB {S} Rd, Rs, Rn
            2 => add_op(rs_val, operand, self.registers.get_carry()), // ADD {S} Rd, Rs, #nn
            3 => sub_op(rs_val, operand, self.registers.get_carry()), // SUB {S} Rd, Rs, #nn
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
            return cycles;
        }

        let arith_op = match op {
            1 => sub_op,
            2 => add_op,
            3 => sub_op,
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

    // COMPLETE
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
        N,Z     for  AND,EOR,TST,ORR,BIC,MVN

        1S      for  AND,EOR,ADC,SBC,TST,NEG,CMP,CMN,ORR,BIC,MVN
        1S+1I   for  LSL,LSR,ASR,ROR
        1S+mI   for  MUL on ARMv4 (m=1..4; depending on MSBs of incoming Rd value)
        1S+mI   for  MUL on ARMv5 (m=3; fucking slow, no matter of MSBs of Rd value)
         */

        let cycles = match op {
            0x0 => {
                // AND{S}
                let result = and_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0x1 => {
                // EOR{S}
                let result = eor_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0x2 => {
                // LSL{S}
                let result = lsl_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x3 => {
                // LSR{S}
                let result = lsr_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x4 => {
                // ASR{S}
                let result = asr_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x5 => {
                // ADC{S}
                // TODO: Check whether all others should add with carry as they currently do
                let result = add_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            0x6 => {
                // SBC{S}
                // TODO: Check whether all others should sub with carry as they currently do
                let result = sub_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            0x7 => {
                // ROR{S}
                let result = ror_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                2
            }
            0x8 => {
                // TST
                let result = and_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                1
            }
            0x9 => {
                // NEG{S}
                let result = sub_op(0, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            0xA => {
                // CMP
                let result = sub_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                1
            }
            0xB => {
                // CMN
                let result = add_op(rd_val, rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                1
            }
            0xC => {
                // ORR{S}
                let result = mvn_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0xD => {
                // MUL{S}
                let result = mul_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());

                // 1S + mI (m=1..4; depending on MSBs of incoming Rd value)
                result.cycles()
            }
            0xE => {
                // BIC{S}
                let result = and_op(rd_val, !rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            0xF => {
                // MVN{S}
                let result = mvn_op(rd_val, rs_val);
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result);
                1
            }
            _ => {
                panic!("Invalid ALU operation")
            }
        };

        return cycles;
    }

    // COMPLETE
    pub fn thumb_hi_register_operations_branch_exchange(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(8..10);
        let msb_d = opcode.extract_bit(7);
        let msb_s = opcode.extract_bit(6);
        let rs = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        if !msb_d && !msb_s {
            panic!("Invalid MSBs and MSBd for this operation.");
        };

        let rs_val = self.registers.get_r(rs);

        let cycles = match op {
            0 => {
                // ADD Rd, Rs
                let result = add_op(self.registers.get_r(rd), rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                self.registers.set_r(rd, result.result());
                1
            }
            1 => {
                // CMP Rd, Rs
                let result = sub_op(self.registers.get_r(rd), rs_val, self.registers.get_carry());
                self.registers.set_flags(&result);
                1
            }
            2 => {
                // MOV Rd, Rs
                self.registers.mov(rd, rs_val);
                1
            }
            3 => {
                // BX Rs or BLX Rs
                // For BX:
                if !rs_val.extract_bit(0) {
                    self.registers.set_thumb_state(false);
                    self.registers.set_pc(rs_val & !1); // Align to word boundary (clear bit 0)
                } else {
                    self.registers.set_pc(rs_val); // Set PC directly to Rs
                }

                // For BLX (when `Rd` is 15, as `Rd` is not used)
                if rd == 15 {
                    self.registers.set_lr(self.registers.get_visible_pc() + 4);
                    self.registers.set_pc(rs_val);
                }

                2
            }
            _ => {
                panic!("Invalid op for thumb_hi_register_operations_branch_exchange")
            }
        };

        cycles
    }

    // COMPLETE
    pub fn thumb_pc_relative_load(&mut self, opcode: u16) -> u64 {
        let rd = opcode.extract_bits(8..11) as usize;
        let nn = opcode.extract_bits(0..8) as u32;

        let pc = self.registers.get_visible_pc();
        let address = pc + 4 + nn;
        let data = self.memory.read_u32(address);

        self.registers.set_r(rd, data);

        // 1S + 1N + 1I
        3
    }

    // COMPLETE
    pub fn thumb_load_store_with_register_offset(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(10..12);
        let ro = opcode.extract_bits(6..9) as usize;
        let rb = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        let rd_val = self.registers.get_r(rd);
        let rb_val = self.registers.get_r(rb);
        let ro_val = self.registers.get_r(ro);

        let address = rb_val.wrapping_add(ro_val);

        let cycles = match op {
            0 => {
                // STR (Store 32-bit data)
                self.memory.write_u32(address, rd_val);
                2
            }
            1 => {
                // STRB (Store 8-bit data)
                self.memory.write_u8(address, rd_val as u8);
                2
            }
            2 => {
                // LDR (Load 32-bit data)
                let data = self.memory.read_u32(address);
                self.registers.set_r(rd, data);
                3
            }
            3 => {
                // LDRB (Load 8-bit data)
                let data = self.memory.read_u8(address) as u32;
                self.registers.set_r(rd, data);
                3
            }
            _ => {
                panic!("Invalid opcode type in load/store with register offset instruction");
            }
        };

        // 1S+1N+1I for LDR, or 2N for STR
        cycles
    }

    // COMPLETE
    pub fn thumb_load_store_sign_extended_byte_halfword(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(10..12);
        let ro = opcode.extract_bits(6..9) as usize;
        let rb = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        let rd_val = self.registers.get_r(rd);
        let rb_val = self.registers.get_r(rb);
        let ro_val = self.registers.get_r(ro);

        let address = rb_val.wrapping_add(ro_val);

        let cycles = match op {
            0 => {
                // STRH (Store Halfword)
                self.memory.write_u16(address, rd_val as u16);
                2
            }
            1 => {
                // LDSB (Load Sign-Extended Byte)
                let data = self.memory.read_u8(address) as i8 as u32;
                self.registers.set_r(rd, data);
                3
            }
            2 => {
                // LDRH (Load Zero-Extended Halfword)
                let data = self.memory.read_u16(address) as u32;
                self.registers.set_r(rd, data);
                3
            }
            3 => {
                // LDSH (Load Sign-Extended Halfword)
                let data = self.memory.read_u16(address) as i16 as u32;
                self.registers.set_r(rd, data);
                3
            }
            _ => {
                panic!("Invalid opcode type in load/store sign-extended byte/halfword instruction");
            }
        };

        // 1S+1N+1I for LDR, or 2N for STR
        cycles
    }

    // COMPLETE
    pub fn thumb_load_store_with_immediate_offset(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bits(11..13);
        let nn = opcode.extract_bits(6..9) as u32;
        let rb = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        let rd_val = self.registers.get_r(rd);
        let rb_val = self.registers.get_r(rb);
        let address = rb_val.wrapping_add(nn);

        let cycles = match op {
            0 => {
                // STR (Store 32-bit)
                self.memory.write_u32(address, rd_val);
                2
            }
            1 => {
                // LDR (Load 32-bit)
                let data = self.memory.read_u32(address);
                self.registers.set_r(rd, data);
                3
            }
            2 => {
                // STRB (Store 8-bit)
                self.memory.write_u8(address, rd_val as u8);
                2
            }
            3 => {
                // LDRB (Load 8-bit)
                let data = self.memory.read_u8(address) as u32;
                self.registers.set_r(rd, data);
                3
            }
            _ => {
                panic!("Invalid opcode type in load/store with immediate offset instruction");
            }
        };

        // 1S+1N+1I for LDR, or 2N for STR
        cycles
    }

    // COMPLETE
    pub fn thumb_load_store_halfword(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let nn = opcode.extract_bits(6..11) as u32;
        let rb = opcode.extract_bits(3..6) as usize;
        let rd = opcode.extract_bits(0..3) as usize;

        let rd_val = self.registers.get_r(rd);
        let rb_val = self.registers.get_r(rb);
        let address = rb_val.wrapping_add(nn * 2);

        let cycles = match op {
            false => {
                // STRH (Store 16-bit Halfword)
                self.memory.write_u16(address, rd_val as u16);
                2
            }
            true => {
                // LDRH (Load 16-bit Halfword)
                let data = self.memory.read_u16(address) as u32;
                self.registers.set_r(rd, data);
                3
            }
        };

        // 1S+1N+1I for LDR, or 2N for STR
        cycles
    }

    // COMPLETE
    pub fn thumb_sp_relative_load_store(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let rd = opcode.extract_bits(8..11) as usize;
        let nn = opcode.extract_bits(0..8) as u32;

        let sp_val = self.registers.get_sp();
        let address = sp_val.wrapping_add(nn);

        let cycles = match op {
            false => {
                // STR (Store 32-bit)
                let rd_val = self.registers.get_r(rd);
                self.memory.write_u32(address, rd_val);
                2
            }
            true => {
                // LDR (Load 32-bit)
                let data = self.memory.read_u32(address);
                self.registers.set_r(rd, data);
                3
            }
        };

        // 1S+1N+1I for LDR, or 2N for STR
        cycles
    }

    // COMPLETE
    pub fn thumb_get_relative_address(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let rd = opcode.extract_bits(8..11) as usize;
        let nn = opcode.extract_bits(0..8) as u32;

        let address = match op {
            false => {
                // ADD Rd, PC, #nn
                let pc = self.registers.get_pc();
                (pc.wrapping_add(4) & !2).wrapping_add(nn)
            }
            true => {
                // ADD Rd, SP, #nn
                let sp = self.registers.get_sp();
                sp.wrapping_add(nn)
            }
        };

        self.registers.set_r(rd, address);

        // 1S
        1
    }

    // COMPLETE
    pub fn thumb_add_offset_to_stack_pointer(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(7);
        let nn = opcode.extract_bits(0..7) as u32;

        let sp_val = self.registers.get_sp();

        let new_sp = match op {
            false => {
                // ADD SP, #nn (SP = SP + nn)
                sp_val.wrapping_add(nn)
            }
            true => {
                // ADD SP, #-nn or SUB SP, #nn (SP = SP - nn)
                sp_val.wrapping_sub(nn)
            }
        };

        self.registers.set_sp(new_sp);

        // 1S
        1
    }

    pub fn thumb_push_pop_registers(&mut self, opcode: u16) -> u64 {
        let op = opcode.extract_bit(11);
        let pc_lr = opcode.extract_bit(8);
        let r_list = opcode.extract_bits(0..8) as u8;

        if op {
            self.thumb_pop(pc_lr, r_list)
        } else {
            self.thumb_push(pc_lr, r_list)
        }
    }

    // COMPLETE
    pub fn thumb_multiple_load_store(&mut self, opcode: u16) -> u64 {
        /*
           TODO:
           Strange Effects on Invalid Rlist's
           Empty Rlist: R15 loaded/stored (ARMv4 only), and Rb=Rb+40h (ARMv4-v5).
           Writeback with Rb included in Rlist: Store OLD base if Rb is FIRST entry in Rlist, otherwise store NEW base (STM/ARMv4),
           always store OLD base (STM/ARMv5), no writeback (LDM/ARMv4/ARMv5; at this point, THUMB opcodes work different than ARM opcodes).
        */
        let op = opcode.extract_bit(11);
        let rb = opcode.extract_bits(8..11) as usize;
        let r_list = opcode.extract_bits(0..8);

        let mut base_addr = self.registers.get_r(rb);
        let num_regs = r_list.count_ones();

        // Iterate through the registers in Rlist (R0 to R7)
        for i in 0..8 {
            if (r_list & (1 << i)) != 0 {
                if op {
                    // LDMIA (Load Multiple Increment After)
                    let value = self.memory.read_u32(base_addr);
                    self.registers.set_r(i, value);
                    base_addr += 4; // Increment the base register after loading
                } else {
                    // STMIA (Store Multiple Increment After)
                    self.memory.write_u32(base_addr, self.registers.get_r(i));
                    base_addr += 4; // Increment the base register after storing
                }
            }
        }

        self.registers.set_r(rb, base_addr);

        if op {
            num_regs as u64 + 2 // LDM: nS + 1N + 1I
        } else {
            (num_regs - 1) as u64 + 2 // STM: (n-1)S + 2N
        }
    }

    // COMPLETE
    pub fn thumb_unconditional_branch(&mut self, opcode: u16) -> u64 {
        let signed_offset_11 = opcode.extract_bits(0..11);
        let signed_offset = ((signed_offset_11 as i32) << 21) >> 21;
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
        let signed_offset = ((signed_offset_8 as i32) << 24) >> 24;
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
        if !opcode.extract_bit(12) {
            self.registers.set_thumb_state(false);
        }

        // 2S + 1N
        3
    }

    // COMPLETE
    pub fn thumb_software_interrupt(&mut self, opcode: u16) -> u64 {
        let comment_field = opcode.extract_bits(0..8) as u8;

        let pc = self.registers.get_pc();
        let cpsr = self.registers.get_cpsr();
        // TODO:
        let r14_svc = pc + 2; // Return address for SVC
        let spsr_svc = cpsr; // Save the CPSR for later restoration

        // Switch to ARM mode, enter Supervisor mode (SVC)
        self.registers.set_cpsr(0x13); // Set CPSR to Supervisor mode (SVC)
        self.registers.set_pc(0x8); // Jump to SWI vector (address 0x08)

        // Optionally store the comment field in a register or memory for handling
        // This is where your SWI handler logic would process the comment field
        self.registers.set_r(0, comment_field as u32);

        // 2S+1N
        3
    }

    // COMPLETE
    pub fn thumb_breakpoint(&mut self, opcode: u16) -> u64 {
        let comment_field = opcode.extract_bits(0..8) as u8;

        let pc = self.registers.get_pc();
        let cpsr = self.registers.get_cpsr();
        // TODO:
        let r14_abt = pc + 4; // Return address for Abort mode
        let spsr_abt = cpsr; // Save the CPSR for later restoration

        // Switch to ARM mode, enter Abort mode
        self.registers.set_cpsr(0x17); // Set CPSR to Abort mode
        self.registers.set_pc(0xC); // Jump to BKPT vector (address 0x0C)

        self.registers.set_r(0, comment_field as u32); // This is where you might process the comment field

        // 2S+1N
        3
    }

    fn thumb_push(&mut self, lr: bool, r_list: u8) -> u64 {
        let mut sp = self.registers.get_sp();
        let mut cycles = 0;

        let num_regs = r_list.count_ones();
        sp = sp - 4 * num_regs;

        if lr {
            sp = sp - 4;
        }

        self.registers.set_sp(sp);
        let mut addr = sp;

        for i in 0..8 {
            if (r_list & (1 << i)) != 0 {
                self.memory.write_u32(addr, self.registers.get_r(i));
                addr += 4;
            }
        }

        if lr {
            self.memory.write_u32(addr, self.registers.get_lr());
        }

        // TODO: Perform any necessary memory prefetching (if applicable)
        self.store_prefetch();

        // (n-1)S+2N
        (num_regs as u64 - 1) + 2
    }

    fn thumb_pop(&mut self, pc: bool, r_list: u8) -> u64 {
        let mut sp = self.registers.get_sp();
        let num_regs = r_list.count_ones();

        for i in 0..8 {
            if (r_list & (1 << i)) != 0 {
                let val = self.memory.read_u32(sp);
                self.registers.set_r(i, val);
                sp += 4;
            }
        }

        if pc {
            let val = self.memory.read_u32(sp);
            self.registers.set_pc(val);
            sp += 4;
        }

        self.registers.set_sp(sp);

        (num_regs as u64) + 2
    }
}
