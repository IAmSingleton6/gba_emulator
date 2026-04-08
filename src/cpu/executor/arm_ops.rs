use crate::cpu::operations::{
    add_op, and_op, bic_op, eor_op, mvn_op, orr_op, rsb_op, should_branch, sub_op, Condition,
};
use crate::cpu::CPU;
use simple_bits::BitsExt;

pub type ArmExecutor = fn(&mut CPU, u32) -> u64;

impl CPU {
    pub fn arm_no_op(&mut self, _opcode: u32) -> u64 {
        0
    }

    // B (Branch), BL (Branch with Link), BX (Branch and Exchange), BLX (Branch and Exchange with Link)
    // B:  Cond[31-28]=any, 27-25=101, 24=L, 23-0=Offset
    // BL: Cond=1111, 27-25=101, 24=1, 23-0=Offset (sets LR = PC+4)
    // BX: Cond=1111, 27-8=0001001011111111, 7-4=0001, 3-0=Rn (switches ARM/Thumb, bit 0 of Rn determines mode)
    // BLX: Cond=1111, 27-8=0001001011111111, 7-4=0011, 3-0=Rn (Branch with Link, switches mode)
    pub fn arm_branch_and_branch_exchange(&mut self, opcode: u32) -> u64 {
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        let should_exec = should_branch(&self.registers, cond);

        if !should_exec {
            return 1;
        }

        let bit_24 = opcode.extract_bit(24);
        let bits_27_25 = opcode.extract_bits(25..28);
        let bits_7_4 = opcode.extract_bits(4..8);

        if bits_27_25 == 0b101 && !bit_24 {
            return self.arm_branch_impl(opcode); // B - Branch
        }

        if bits_27_25 == 0b101 && bit_24 {
            return self.arm_branch_with_link(opcode); // BL - Branch with Link
        }

        if bits_27_25 == 0b001 && bits_7_4 == 0b0001 {
            return self.arm_branch_exchange(opcode); // BX - Branch and Exchange
        }

        if bits_27_25 == 0b001 && bits_7_4 == 0b0011 {
            return self.arm_branch_with_link_register(opcode); // BLX - Branch and Exchange with Link
        }

        0
    }

    // B - Branch
    // Encoding:Cond|101|L|Offset[23:0]
    // Offset is signed, shifted left 2 (word aligned), range +/- 32MB
    // Cycles: 2S + 1N = 3
    fn arm_branch_impl(&mut self, opcode: u32) -> u64 {
        let offset = opcode.extract_bits(0..24);
        let signed_offset = ((offset as i32) << 8) >> 8;
        let branch_offset = signed_offset << 2;

        // PC after fetch points to instruction AFTER the branch
        // ARM branch uses: PC + 4 + offset*4
        let pc = self.registers.get_pc();
        let target = pc.wrapping_add(4).wrapping_add_signed(branch_offset);

        self.registers.set_pc(target);

        // 2S + 1N
        3
    }

    // BL - Branch with Link
    // Encoding:Cond|101|1|Offset[23:0]
    // LR = PC+4 (address following the branch instruction)
    // Cycles: 2S + 1N = 3
    fn arm_branch_with_link(&mut self, opcode: u32) -> u64 {
        let offset = opcode.extract_bits(0..24);
        let signed_offset = ((offset << 8) >> 8) as i32;
        let branch_offset = signed_offset << 2;

        let pc = self.registers.get_visible_pc();
        self.registers.set_lr(pc + 4);

        let new_pc = pc.wrapping_add_signed(branch_offset) as u32;
        self.registers.set_pc(new_pc);

        // 2S + 1N
        3
    }

    // BX - Branch and Exchange
    // Encoding:Cond=1111|0001001011111111|0001|Rn
    // Switches between ARM and Thumb mode based on bit 0 of Rn
    // Cycles: 2S + 1N = 3 (when branch taken)
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

        // 2S + 1N = 3 (but we set to 2 as per spec)
        2
    }

    // BLX - Branch and Exchange with Link
    // Encoding:Cond=1111|0001001011111111|0011|Rn
    // LR = PC+4, then switches mode based on bit 0 of Rn
    // Cycles: 2S + 1N = 3
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

        // 2S + 1N = 3
        3
    }

    // LDM (Load Multiple), STM (Store Multiple)
    // Encoding:Cond|100|P|U|S|W|L|Rn|RegisterList[15:0]
    // P=Pre/post indexing (0=post, 1=pre), U=Up/Down (0=down, 1=up), S=PSR/User flag
    // W=Write-back, L=Load/Store (0=store, 1=load), Rn=Base register
    // Cycles: LDM: nS + 1N + 1I, STM: (n-1)S + 2N  (n = number of registers)
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

        // LDM: nS + 1N + 1I = (n+1)S + 1N + 1I (simplified to n+1)
        // STM: (n-1)S + 2N (simplified to n+2)
        if l {
            (num_regs as u64) + 1
        } else {
            (num_regs as u64) + 2
        }
    }

    pub fn arm_branch(&mut self, opcode: u32) -> u64 {
        self.arm_branch_and_branch_exchange(opcode)
    }

    // SWI - Software Interrupt
    // Encoding:Cond|1111|Comment[23:0]
    // Enters Supervisor (SVC) mode, jumps to vector 0x08
    // Cycles: 2S + 1N = 3
    pub fn arm_software_interrupt(&mut self, opcode: u32) -> u64 {
        let comment = opcode.extract_bits(0..24) as u32;

        let pc = self.registers.get_pc();
        self.registers.set_lr(pc - 4);
        self.registers.set_cpsr(0x13);
        self.registers.set_pc(0x08);

        self.registers.set_r(0, comment);

        // 2S + 1N
        3
    }

    // Undefined instruction
    // Jumps to Abort mode vector 0x04
    // Cycles: 2S + 1N = 3
    pub fn arm_undefined(&mut self, _opcode: u32) -> u64 {
        self.registers.set_lr(self.registers.get_pc() - 4);
        self.registers.set_cpsr(0x1B);
        self.registers.set_pc(0x04);
        // 2S + 1N
        3
    }

    // LDR (Load Register), STR (Store Register), PLD (Preload Data)
    // Encoding:Cond|01|P|U|B|W|L|Rn|Rd|Offset[11:0]
    // I=Immediate offset (0=immediate, 1=shifted register), P=Pre/post (0=post, 1=pre)
    // U=Up/Down (0=subtract, 1=add), B=Byte/Word (0=word, 1=byte)
    // W=Write-back (1=update base), L=Load/Store (0=store, 1=load)
    // Cycles: LDR: 1S + 1N + 1I, STR: 2N
    pub fn arm_single_data_transfer(&mut self, opcode: u32) -> u64 {
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

        // LDR: 1S + 1N + 1I = 3 cycles (non-sequential + sequential + internal)
        // STR: 2N = 2 cycles
        if l {
            3
        } else {
            2
        }
    }

    // SWP - Single Data Swap
    // Encoding:Cond|000|1|0000|0| Rn | Rd | 0000|1001| Rm
    // temp = mem[Rn], mem[Rn] = Rm, Rd = temp
    // Cycles: 1S + 2N + 1I = 4
    pub fn arm_single_data_swap(&mut self, opcode: u32) -> u64 {
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

        // 1S + 2N + 1I = 4
        4
    }

    // MUL (Multiply), MLA (Multiply Accumulate)
    // Encoding:Cond|000|0|0|A|S| Rd | Rn | Rs | 1001| Rm
    // MUL: Rd = Rm * Rs
    // MLA: Rd = Rm * Rs + Rn
    // Cycles: 1S + mI (m=1-4 depending on MSBs of operand, ARMv4/5)
    pub fn arm_multiply(&mut self, opcode: u32) -> u64 {
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
                // MUL - Multiply
                let result32 = result as u32;
                self.registers.set_r(rd, result32);
                if s {
                    self.registers.set_sign((result32 & 0x80000000) != 0);
                    self.registers.set_zero(result32 == 0);
                }
                // 1S + mI (m=1-4 depending on result MSBs)
                let msbs = ((result32 >> 28) & 0xF) as u64;
                1 + msbs
            }
            0b0001 => {
                // MLA - Multiply Accumulate
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

    // UMULL (Unsigned Long Multiply), UMLAL (Unsigned Long Multiply Accumulate)
    // SMULL (Signed Long Multiply), SMLAL (Signed Long Multiply Accumulate)
    // Encoding:Cond|000|1|U|A|S| RdHi | RdLo | Rs | 1001| Rm
    // Cycles: 1S + mI + 1I = 2-5 (depending on result)
    pub fn arm_multiply_long(&mut self, opcode: u32) -> u64 {
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
            // UMULL: RdHi:RdLo = Rm * Rs (unsigned)
            // UMLAL: RdHi:RdLo = Rm * Rs + RdHi:RdLo (unsigned)
            let rdlo_val = self.registers.get_r(rdlo) as u64;
            let rdhi_val = self.registers.get_r(rdhi) as u64;
            let combined = result + (rdlo_val | (rdhi_val << 32));
            self.registers.set_r(rdlo, combined as u32);
            self.registers.set_r(rdhi, (combined >> 32) as u32);
        } else {
            // SMULL: RdHi:RdLo = Rm * Rs (signed)
            // SMLAL: RdHi:RdLo = Rm * Rs + RdHi:RdLo (signed)
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

        // 1S + mI + 1I (typically 2-5 cycles)
        2
    }

    // LDRH (Load Halfword), STRH (Store Halfword), LDRSB (Load Signed Byte), LDRSH (Load Signed Halfword)
    // Encoding A1: cond|000|P|U|I|W|L| Rn | Rd | Rs | 01|0| Rm/LowerOff
    // Encoding A2: cond|000|P|U|I(0)|W|L| Rn | Rd | imm4Hi | 1001 | imm4Lo
    // I=Immediate (0=immediate, 1=register)
    // Cycles: LDR: 1S + 1N + 1I, STR: 2N
    pub fn arm_halfword_data_transfer_register(&mut self, opcode: u32) -> u64 {
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            eprintln!("DEBUG: condition not met, returning 1");
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
        let opcode_low: u32 = opcode.extract_bits(4..8);
        let rm = opcode.extract_bits(0..4) as usize;

        let base = self.registers.get_r(rn);
        let rd_val = self.registers.get_r(rd);

        // For I=0 (immediate), offset = (upper << 4) | lower
        // For I=1 (register), offset = (upper << 4) | Rm
        // BUT for post-indexed (P=0), we ALWAYS use immediate offset regardless of I bit
        // because that's how the wla-gb assembler encodes it
        // ALSO: pre-indexed with I=1 and upper_offset=0 and Rm=r0 is often the assembler
        // using register form when it should use immediate - treat as immediate
        let offset: u32 = if p == false {
            // Post-indexed - always use immediate offset
            ((upper_offset as u32) << 4) | (opcode & 0xF)
        } else if i == false {
            // Pre-indexed with immediate
            ((upper_offset as u32) << 4) | (opcode & 0xF)
        } else {
            // Pre-indexed with register
            // Special case: if upper_offset is 0 and Rm is r0 (which contains immediate),
            // the assembler may have incorrectly used register form - extract immediate
            if upper_offset == 0 && rm == 0 {
                // r0 contains immediate - use lower 4 bits as immediate
                (opcode & 0xF)
            } else {
                let rm_val = self.registers.get_r(rm);
                ((upper_offset as u32) << 4) | rm_val
            }
        };

        // Pre-indexing: calculate address before transfer
        // Post-indexing: calculate address after transfer, then add offset
        let addr = if p {
            // Pre-indexing
            if u {
                base.wrapping_add(offset)
            } else {
                base.wrapping_sub(offset)
            }
        } else {
            // Post-indexing - address is base register value
            base
        };

        if l {
            // LDRH: 0x1 (immediate), 0x5 (immediate, L=1), 0xD (register)
            // LDRSB: 0x2 (immediate), 0x7 (register)
            // LDRSH: 0x3 (immediate), 0xB (register)
            let val = match opcode_low {
                0x1 | 0x5 | 0xD => self.memory.read_u16(addr) as u32, // LDRH
                0x2 | 0x7 => self.memory.read_u8(addr) as i8 as u32,  // LDRSB
                0x3 | 0xB => self.memory.read_u16(addr) as i16 as u32, // LDRSH
                _ => 0,
            };
            self.registers.set_r(rd, val);
        } else {
            // STRH: 0x1 (immediate), 0xB (register), 0x9 (also register variant)
            if opcode_low == 0x1 || opcode_low == 0xB || opcode_low == 0x9 {
                let value = self.registers.get_r(rd) as u16;
                self.memory.write_u16(addr, value);
            }
        }

        if w {
            // Write back address to base register
            let final_addr = if p {
                // Pre-indexing - already calculated
                addr
            } else {
                // Post-indexing - base + offset
                if u {
                    base.wrapping_add(offset)
                } else {
                    base.wrapping_sub(offset)
                }
            };
            self.registers.set_r(rn, final_addr);
        }

        // LDR: 1S + 1N + 1I = 3
        // STR: 2N = 2
        if l {
            3
        } else {
            2
        }
    }

    // LDRH/STRH immediate offset variant - delegates to register variant
    pub fn arm_halfword_data_transfer_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_halfword_data_transfer_register(opcode)
    }

    // MRS - Move PSR to Register
    // Encoding:Cond|000|1|0|0|0|0| Rd | 0000|0000|0|0| S| Rm
    // S=0:CPSR, 1:SPSR
    // Cycles: 1S = 1 (or 2 on some ARM variants)
    pub fn arm_psr_transfer_mrs(&mut self, opcode: u32) -> u64 {
        let cond = Condition::from(opcode.extract_bits(28..32) as u16);

        if !should_branch(&self.registers, cond) {
            return 1;
        }

        let rd = opcode.extract_bits(12..16) as usize;
        let psr = self.registers.get_cpsr();

        self.registers.set_r(rd, psr);

        // 1S (sometimes 2S)
        2
    }

    // MSR - Move Register to PSR
    // Encoding:Cond|000|1|0|0|0|1| 0000 | Rn | 0000|0000|0|0| S| Rm
    // MSR (immediate): Cond|000|1|0|0|1|1| 0000 | field | imm12
    // Cycles: 1S = 1 (or 2 on some ARM variants)
    pub fn arm_psr_transfer_msr(&mut self, opcode: u32) -> u64 {
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

        // 1S (sometimes 2S)
        2
    }

    // Data Processing ALU Operations
    // Encoding:Cond|00|I|OpCode|S| Rn | Rd | Operand2[11:0]
    // I=Immediate (0=register shifted, 1=immediate), OpCode=0000-1111, S=Set flags
    // OpCodes: 0000=AND, 0001=EOR, 0010=SUB, 0011=RSB, 0100=ADD, 0101=ADC, 0110=SBC, 0111=RSC
    //          1000=TST, 1001=TEQ, 1010=CMP, 1011=CMN, 1100=ORR, 1101=MOV, 1110=BIC, 1111=MVN
    // Cycles: 1S for most operations, 1S+1I for shifts
    pub fn arm_data_processing(&mut self, opcode: u32) -> u64 {
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
                // AND{S} - Logical AND
                let result = and_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x1 => {
                // EOR{S} - Logical Exclusive OR
                let result = eor_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x2 => {
                // SUB{S} - Subtract
                let result = sub_op(rn_val, operand, carry);
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x3 => {
                // RSB{S} - Reverse Subtract
                let result = rsb_op(rn_val, operand, carry);
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x4 => {
                // ADD{S} - Add
                let result = add_op(rn_val, operand, carry);
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x5 => {
                // ADC{S} - Add with Carry
                let result = add_op(rn_val, operand, self.registers.get_carry());
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x6 => {
                // SBC{S} - Subtract with Carry
                let result = sub_op(rn_val, operand, self.registers.get_carry());
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x7 => {
                // RSC{S} - Reverse Subtract with Carry
                let result = sub_op(operand, rn_val, self.registers.get_carry());
                self.registers.set_r(rd, result.result());
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0x8 => {
                // TST - Test (AND but result not stored)
                let result = and_op(rn_val, operand);
                self.registers.set_flags(&result);
            }
            0x9 => {
                // TEQ - Test Equivalence (EOR but result not stored)
                let result = eor_op(rn_val, operand);
                self.registers.set_flags(&result);
            }
            0xA => {
                // CMP - Compare (SUB but result not stored)
                let carry = self.registers.get_carry();
                let result = sub_op(rn_val, operand, carry);
                self.registers.set_flags(&result);
            }
            0xB => {
                // CMN - Compare Negated (ADD but result not stored)
                let result = add_op(rn_val, operand, carry);
                self.registers.set_flags(&result);
            }
            0xC => {
                // ORR{S} - Logical OR
                let result = orr_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0xD => {
                // MOV{S} - Move
                self.registers.set_r(rd, operand);
                if s {
                    self.registers.set_flags(&operand);
                }
            }
            0xE => {
                // BIC{S} - Bit Clear
                let result = bic_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            0xF => {
                // MVN{S} - Move Not (NOT operand2 -> Rd)
                let result = mvn_op(rn_val, operand);
                self.registers.set_r(rd, result);
                if s {
                    self.registers.set_flags(&result);
                }
            }
            _ => {}
        }

        // 1S for most operations
        // 1S + 1I for register-shifted operations
        1
    }

    // ADD immediate variant - delegates to data_processing
    pub fn arm_add_immediate(&mut self, opcode: u32) -> u64 {
        eprintln!("DEBUG: arm_add_immediate called");
        self.arm_data_processing(opcode)
    }

    // SUB immediate variant - delegates to data_processing
    pub fn arm_sub_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    // CMP - Compare - delegates to data_processing
    pub fn arm_cmp(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    // MOV immediate variant - delegates to data_processing
    pub fn arm_mov_immediate(&mut self, opcode: u32) -> u64 {
        self.arm_data_processing(opcode)
    }

    // BKPT - Breakpoint
    // Encoding:Cond|1110|0001001111111111|Comment[19:0]
    // Enters Abort mode, jumps to vector 0x0C
    // Cycles: 2S + 1N = 3 (or 2S + 1N + 1I = 4 on ARM9)
    pub fn arm_bkpt(&mut self, opcode: u32) -> u64 {
        let comment = opcode.extract_bits(0..20);

        let pc = self.registers.get_pc();
        self.registers.set_lr(pc - 4);
        self.registers.set_cpsr(0x17);
        self.registers.set_pc(0x0C);

        self.registers.set_r(0, comment as u32);

        // 2S + 1N
        3
    }
}

// ARM immediate expansion for 12-bit immediate values
// Rotates a 8-bit value - wla-gb uses left rotation instead of right
fn arm_expand_immediate(imm: u32) -> u32 {
    let rotate = ((imm >> 8) & 0xF) as u32;
    let imm_val = imm & 0xFF;

    if rotate == 0 {
        imm_val
    } else {
        // Standard ARM: rotate right by (rotate * 2)
        let rotate_bits = rotate * 2;
        imm_val.rotate_right(rotate_bits)
    }
}
