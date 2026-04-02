use crate::cpu::operations::{add_op, and_op, bic_op, eor_op, mvn_op, orr_op, rsb_op, should_branch, sub_op, Condition};
use crate::cpu::CPU;
use simple_bits::BitsExt;

pub type ArmExecutor = fn(&mut CPU, u32) -> u64;

impl CPU {
    pub fn arm_no_op(&mut self, _opcode: u32) -> u64 {
        0
    }

    pub fn arm_branch_and_branch_exchange(&mut self, opcode: u32) -> u64 {
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let bit_24 = opcode.extract_bit(24);
        let bits_27_25 = opcode.extract_bits(25..28);
        let bits_7_4 = opcode.extract_bits(4..8);

        if bits_27_25 == 0b101 && !bit_24 {
            return self.arm_branch_impl(opcode);
        }

        if bits_27_25 == 0b101 && bit_24 {
            return self.arm_branch_with_link(opcode);
        }

        if bits_27_25 == 0b001 && bits_7_4 == 0b0001 {
            return self.arm_branch_exchange(opcode);
        }

        if bits_27_25 == 0b001 && bits_7_4 == 0b0011 {
            return self.arm_branch_with_link_register(opcode);
        }

        0
    }

    fn arm_branch_impl(&mut self, opcode: u32) -> u64 {
        let offset = opcode.extract_bits(0..24);
        let signed_offset = ((offset << 8) >> 8) as i32;
        let branch_offset = signed_offset << 2;

        let pc = self.registers.get_visible_pc();
        let new_pc = pc.wrapping_add_signed(branch_offset) as u32;

        self.registers.set_pc(new_pc);

        3
    }

    fn arm_branch_with_link(&mut self, opcode: u32) -> u64 {
        let offset = opcode.extract_bits(0..24);
        let signed_offset = ((offset << 8) >> 8) as i32;
        let branch_offset = signed_offset << 2;

        let pc = self.registers.get_visible_pc();
        self.registers.set_lr(pc + 4);

        let new_pc = pc.wrapping_add_signed(branch_offset) as u32;
        self.registers.set_pc(new_pc);

        3
    }

    fn arm_branch_exchange(&mut self, opcode: u32) -> u64 {
        let rn = opcode.extract_bits(0..4) as usize;
        let rn_val = self.registers.get_r(rn);

        if !rn_val.extract_bit(0) {
            self.registers.set_thumb_state(false);
            self.registers.set_pc(rn_val & !1);
        } else {
            self.registers.set_thumb_state(true);
            self.registers.set_pc(rn_val & !1);
        }

        2
    }

    fn arm_branch_with_link_register(&mut self, opcode: u32) -> u64 {
        let rn = opcode.extract_bits(0..4) as usize;
        let rn_val = self.registers.get_r(rn);

        let pc = self.registers.get_visible_pc();
        self.registers.set_lr(pc + 4);

        if !rn_val.extract_bit(0) {
            self.registers.set_thumb_state(false);
            self.registers.set_pc(rn_val & !1);
        } else {
            self.registers.set_thumb_state(true);
            self.registers.set_pc(rn_val & !1);
        }

        3
    }

    pub fn arm_block_data_transfer(&mut self, opcode: u32) -> u64 {
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let p = opcode.extract_bit(24);
        let u = opcode.extract_bit(23);
        let s = opcode.extract_bit(22);
        let w = opcode.extract_bit(21);
        let l = opcode.extract_bit(20);
        let rn = opcode.extract_bits(16..20) as usize;
        let rlist = opcode.extract_bits(0..16) as u16;

        let base_addr = self.registers.get_r(rn);
        let num_regs = rlist.count_ones();
        let mut addr = if u {
            base_addr
        } else {
            base_addr.wrapping_sub(num_regs as u32 * 4)
        };

        let start_addr = addr;

        if l {
            for i in 0..16 {
                if (rlist & (1 << i)) != 0 {
                    let val = self.memory.read_u32(addr);
                    self.registers.set_r(i, val);
                    addr = addr.wrapping_add(4);
                }
            }
        } else {
            for i in 0..16 {
                if (rlist & (1 << i)) != 0 {
                    self.memory.write_u32(addr, self.registers.get_r(i));
                    addr = addr.wrapping_add(4);
                }
            }
        }

        if w {
            let final_addr = if u {
                start_addr.wrapping_add(num_regs as u32 * 4)
            } else {
                start_addr
            };
            self.registers.set_r(rn, final_addr);
        }

        if l {
            (num_regs as u64) + 1
        } else {
            (num_regs as u64) + 2
        }
    }

    pub fn arm_branch(&mut self, opcode: u32) -> u64 {
        self.arm_branch_and_branch_exchange(opcode)
    }

    pub fn arm_software_interrupt(&mut self, opcode: u32) -> u64 {
        let comment = opcode.extract_bits(0..24) as u32;

        let pc = self.registers.get_pc();
        self.registers.set_lr(pc - 4);
        self.registers.set_cpsr(0x13);
        self.registers.set_pc(0x08);

        self.registers.set_r(0, comment);

        3
    }

    pub fn arm_undefined(&mut self, _opcode: u32) -> u64 {
        self.registers.set_lr(self.registers.get_pc() - 4);
        self.registers.set_cpsr(0x1B);
        self.registers.set_pc(0x04);
        3
    }

    pub fn arm_single_data_transfer(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let i = opcode.extract_bit(25);
        let p = opcode.extract_bit(24);
        let u = opcode.extract_bit(23);
        let b = opcode.extract_bit(22);
        let w = opcode.extract_bit(21);
        let l = opcode.extract_bit(20);
        let rn = opcode.extract_bits(16..20) as usize;
        let rd = opcode.extract_bits(12..16) as usize;

        let base = self.registers.get_r(rn);
        let rd_val = self.registers.get_r(rd);

        let offset = if i == false {
            opcode.extract_bits(0..12)
        } else {
            let shift_imm = opcode.extract_bits(7..12) as u32;
            let shift_type = opcode.extract_bits(5..7);
            let rm_val = self.registers.get_r(opcode.extract_bits(0..4) as usize);

            match shift_type {
                0b00 => rm_val << shift_imm,
                0b01 => rm_val >> if shift_imm == 0 { 32 } else { shift_imm },
                0b10 => ((rm_val as i32) >> if shift_imm == 0 { 32 } else { shift_imm }) as u32,
                0b11 => rm_val.rotate_right(if shift_imm == 0 { 1 } else { shift_imm }),
                _ => rm_val,
            }
        };

        let addr = if u {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };

        if l {
            let value = if b {
                self.memory.read_u8(addr) as u32
            } else {
                self.memory.read_u32(addr)
            };
            self.registers.set_r(rd, value);
        } else {
            if b {
                self.memory.write_u8(addr, rd_val as u8);
            } else {
                self.memory.write_u32(addr, rd_val);
            }
        }

        if w {
            self.registers.set_r(rn, addr);
        }

        if l {
            3
        } else {
            2
        }
    }

    pub fn arm_single_data_swap(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let rn = opcode.extract_bits(16..20) as usize;
        let rd = opcode.extract_bits(12..16) as usize;
        let rm = opcode.extract_bits(0..4) as usize;

        let addr = self.registers.get_r(rn);
        let rd_val = self.registers.get_r(rd);
        let rm_val = self.registers.get_r(rm);

        let old_val = self.memory.read_u32(addr);
        self.registers.set_r(rd, old_val);
        self.memory.write_u32(addr, rm_val);

        4
    }

    pub fn arm_multiply(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let bits_24_21 = opcode.extract_bits(21..25);
        let s = opcode.extract_bit(20);
        let rd = opcode.extract_bits(16..20) as usize;
        let rn = opcode.extract_bits(12..16) as usize;
        let rs = opcode.extract_bits(8..12) as usize;
        let rm = opcode.extract_bits(0..4) as usize;

        let rm_val = self.registers.get_r(rm);
        let rs_val = self.registers.get_r(rs);

        let result: u64 = (rm_val as u64) * (rs_val as u64);

        match bits_24_21 {
            0b0000 => {
                let result32 = result as u32;
                self.registers.set_r(rd, result32);
                if s {
                    self.registers.set_sign((result32 & 0x80000000) != 0);
                    self.registers.set_zero(result32 == 0);
                }
                let msbs = ((result32 >> 28) & 0xF) as u64;
                1 + msbs
            }
            0b0001 => {
                let rn_val = self.registers.get_r(rn);
                let result32 = (result + rn_val as u64) as u32;
                self.registers.set_r(rd, result32);
                if s {
                    self.registers.set_sign((result32 & 0x80000000) != 0);
                    self.registers.set_zero(result32 == 0);
                }
                let msbs = ((result32 >> 28) & 0xF) as u64;
                1 + msbs
            }
            _ => 1,
        }
    }

    pub fn arm_multiply_long(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let u = opcode.extract_bit(23);
        let s = opcode.extract_bit(20);
        let rdhi = opcode.extract_bits(16..20) as usize;
        let rdlo = opcode.extract_bits(12..16) as usize;
        let rs = opcode.extract_bits(8..12) as usize;
        let rm = opcode.extract_bits(0..4) as usize;

        let rm_val = self.registers.get_r(rm);
        let rs_val = self.registers.get_r(rs);

        let result: u64 = (rm_val as u64) * (rs_val as u64);

        if u {
            let rdlo_val = self.registers.get_r(rdlo) as u64;
            let rdhi_val = self.registers.get_r(rdhi) as u64;
            let combined = result + (rdlo_val | (rdhi_val << 32));
            self.registers.set_r(rdlo, combined as u32);
            self.registers.set_r(rdhi, (combined >> 32) as u32);
        } else {
            let rdlo_val = rm_val as i64;
            let rdhi_val = rs_val as i64;
            let result_val = rdlo_val * rdhi_val;
            self.registers.set_r(rdlo, result_val as u32);
            self.registers.set_r(rdhi, (result_val >> 32) as u32);
        }

        if s {
            let result32 = result as u32;
            self.registers.set_sign((result32 & 0x80000000) != 0);
            self.registers.set_zero(result32 == 0);
        }

        2
    }

    pub fn arm_halfword_data_transfer_register(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let p = opcode.extract_bit(24);
        let u = opcode.extract_bit(23);
        let i = opcode.extract_bit(22);
        let w = opcode.extract_bit(21);
        let l = opcode.extract_bit(20);
        let rn = opcode.extract_bits(16..20) as usize;
        let rd = opcode.extract_bits(12..16) as usize;
        let upper_offset = opcode.extract_bits(8..12);
        let opcode_low = opcode.extract_bits(5..7);
        let rm = opcode.extract_bits(0..4) as usize;

        let base = self.registers.get_r(rn);
        let offset = if i {
            let rm_val = self.registers.get_r(rm);
            ((upper_offset as u32) << 4) | rm_val
        } else {
            ((upper_offset as u32) << 4) | (opcode.extract_bits(0..4) as u32)
        };

        let addr = if u {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };

        if l {
            let val = match opcode_low {
                0b01 => self.memory.read_u16(addr) as u32,
                0b10 => self.memory.read_u8(addr) as i8 as u32,
                0b11 => self.memory.read_u16(addr) as i16 as u32,
                _ => 0,
            };
            self.registers.set_r(rd, val);
        } else {
            if opcode_low == 0b01 {
                self.memory.write_u16(addr, self.registers.get_r(rd) as u16);
            }
        }

        if w {
            self.registers.set_r(rn, addr);
        }

        if l {
            3
        } else {
            2
        }
    }

    pub fn arm_halfword_data_transfer_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_halfword_data_transfer_register(opcode)
    }

    pub fn arm_psr_transfer_mrs(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let rd = opcode.extract_bits(12..16) as usize;
        let psr = self.registers.get_cpsr();

        self.registers.set_r(rd, psr);

        2
    }

    pub fn arm_psr_transfer_msr(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let i = opcode.extract_bit(25);
        let field_mask = opcode.extract_bits(16..20);

        let value = if i {
            arm_expand_immediate(opcode.extract_bits(0..12))
        } else {
            let shift_imm = opcode.extract_bits(7..12) as u32;
            let shift_type = opcode.extract_bits(5..7);
            let rm_val = self.registers.get_r(opcode.extract_bits(0..4) as usize);

            match shift_type {
                0b00 => rm_val << shift_imm,
                0b01 => rm_val >> if shift_imm == 0 { 32 } else { shift_imm },
                0b10 => ((rm_val as i32) >> if shift_imm == 0 { 32 } else { shift_imm }) as u32,
                0b11 => rm_val.rotate_right(if shift_imm == 0 { 1 } else { shift_imm }),
                _ => rm_val,
            }
        };

        let mut cpsr = self.registers.get_cpsr();
        if field_mask.extract_bit(0) {
            cpsr = (cpsr & 0xFFFFFF00) | (value & 0x000000FF);
        }
        if field_mask.extract_bit(1) {
            cpsr = (cpsr & 0xFFFF00FF) | (value & 0x0000FF00);
        }
        if field_mask.extract_bit(2) {
            cpsr = (cpsr & 0xFF00FFFF) | (value & 0x00FF0000);
        }
        if field_mask.extract_bit(3) {
            cpsr = (cpsr & 0x00FFFFFF) | (value & 0xFF000000);
        }
        self.registers.set_cpsr(cpsr);

        2
    }

    pub fn arm_data_processing(&mut self, opcode: u32) -> u64 {
        let cond_bits = opcode.extract_bits(28..32) as u8;
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let i = opcode.extract_bit(25);
        let opcode_val = opcode.extract_bits(21..25) as u8;
        let s = opcode.extract_bit(20);
        let rn = opcode.extract_bits(16..20) as usize;
        let rd = opcode.extract_bits(12..16) as usize;
        let operand2 = opcode.extract_bits(0..12);

        let rn_val = self.registers.get_r(rn);

        let operand = if i {
            arm_expand_immediate(operand2)
        } else {
            let shift_imm = operand2.extract_bits(7..12) as u32;
            let shift_type = operand2.extract_bits(5..7);
            let rm_val = self.registers.get_r(operand2.extract_bits(0..4) as usize);

            match shift_type {
                0b00 => rm_val << shift_imm,
                0b01 => rm_val >> if shift_imm == 0 { 32 } else { shift_imm },
                0b10 => ((rm_val as i32) >> if shift_imm == 0 { 32 } else { shift_imm }) as u32,
                0b11 => rm_val.rotate_right(if shift_imm == 0 { 1 } else { shift_imm }),
                _ => rm_val,
            }
        };

        let carry = self.registers.get_carry();

        match opcode_val {
            0x0 => {
                let result = and_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x1 => {
                let result = eor_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x2 => {
                let result = sub_op(rn_val, operand, carry);
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x3 => {
                let result = rsb_op(rn_val, operand, carry);
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x4 => {
                let result = add_op(rn_val, operand, carry);
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x5 => {
                let result = add_op(rn_val, operand, self.registers.get_carry());
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x6 => {
                let result = sub_op(rn_val, operand, self.registers.get_carry());
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x7 => {
                let result = sub_op(operand, rn_val, self.registers.get_carry());
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x8 => {
                let result = and_op(rn_val, operand);
                self.registers.set_flags(&result);
            }
            0x9 => {
                let result = eor_op(rn_val, operand);
                self.registers.set_flags(&result);
            }
            0xA => {
                let result = sub_op(rn_val, operand, carry);
                self.registers.set_flags(&result);
            }
            0xB => {
                let result = add_op(rn_val, operand, carry);
                self.registers.set_flags(&result);
            }
            0xC => {
                let result = orr_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0xD => {
                self.registers.set_r(rd, operand);
                if s {
                    self.registers.set_flags(&operand);
                }
            }
            0xE => {
                let result = bic_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0xF => {
                let result = mvn_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            _ => {}
        }

        1
    }

    pub fn arm_add_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    pub fn arm_sub_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    pub fn arm_cmp(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    pub fn arm_mov_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    pub fn arm_bkpt(&mut self, opcode: u32) -> u64 {
        let comment = opcode.extract_bits(0..20);

        let pc = self.registers.get_pc();
        self.registers.set_lr(pc - 4);
        self.registers.set_cpsr(0x17);
        self.registers.set_pc(0x0C);

        self.registers.set_r(0, comment as u32);

        3
    }
}

fn arm_expand_immediate(imm: u32) -> u32 {
    let rotate = ((imm >> 8) & 0xF) as u32;
    let imm_val = imm & 0xFF;

    if rotate == 0 {
        imm_val
    } else {
        imm_val.rotate_right(rotate * 2)
    }
}
