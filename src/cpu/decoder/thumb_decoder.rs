use crate::cpu::ThumbExecutor;
use crate::cpu::CPU;

pub fn decode_thumb(opcode: u16) -> ThumbExecutor {
    for format in THUMB_INSTRUCTION_FORMATS {
        if format.matches(opcode as u16) {
            return format.executor;
        }
    }

    CPU::thumb_no_op
}

#[derive(Debug)]
struct ThumbDecoder {
    format: u16,
    mask: u16,
    executor: ThumbExecutor,
}

impl ThumbDecoder {
    fn matches(&self, opcode: u16) -> bool {
        (opcode & self.mask) == self.format
    }
}

const THUMB_INSTRUCTION_FORMATS: [ThumbDecoder; 21] = [
    // Register Operations (ALU, BX)
    ThumbDecoder {
        format: 0b0000_0000_0000_0000,
        mask: 0b1110_0000_0000_0000,
        executor: CPU::thumb_move_shifted_register,
    },
    ThumbDecoder {
        format: 0b0001_1000_0000_0000,
        mask: 0b1111_1000_0000_0000,
        executor: CPU::thumb_add_subtract,
    },
    ThumbDecoder {
        format: 0b0010_0000_0000_0000,
        mask: 0b1110_0000_0000_0000,
        executor: CPU::thumb_move_compare_add_subtract_immediate,
    },
    ThumbDecoder {
        format: 0b0100_0000_0000_0000,
        mask: 0b1111_1100_0000_0000,
        executor: CPU::thumb_alu_operations,
    },
    ThumbDecoder {
        format: 0b0100_0100_0000_0000,
        mask: 0b1111_1100_0000_0000,
        executor: CPU::thumb_hi_register_operations_branch_exchange,
    },
    // Memory Load/Store (LDR/STR)
    ThumbDecoder {
        format: 0b0100_1000_0000_0000,
        mask: 0b1111_1000_0000_0000,
        executor: CPU::thumb_pc_relative_load,
    },
    ThumbDecoder {
        format: 0b0101_0000_0000_0000,
        mask: 0b1111_0010_0000_0000,
        executor: CPU::thumb_load_store_with_register_offset,
    },
    ThumbDecoder {
        format: 0b0101_0010_0000_0000,
        mask: 0b1111_0010_0000_0000,
        executor: CPU::thumb_load_store_sign_extended_byte_halfword,
    },
    ThumbDecoder {
        format: 0b0110_0000_0000_0000,
        mask: 0b1110_0000_0000_0000,
        executor: CPU::thumb_load_store_with_immediate_offset,
    },
    ThumbDecoder {
        format: 0b1000_0000_0000_0000,
        mask: 0b1111_0000_0000_0000,
        executor: CPU::thumb_load_store_halfword,
    },
    ThumbDecoder {
        format: 0b1001_0000_0000_0000,
        mask: 0b1111_0000_0000_0000,
        executor: CPU::thumb_sp_relative_load_store,
    },
    // Memory Addressing (ADD PC/SP)
    ThumbDecoder {
        format: 0b1010_0000_0000_0000,
        mask: 0b1111_0000_0000_0000,
        executor: CPU::thumb_get_relative_address,
    },
    ThumbDecoder {
        format: 0b1011_0000_0000_0000,
        mask: 0b1111_1111_0000_0000,
        executor: CPU::thumb_add_offset_to_stack_pointer,
    },
    // Memory Multiple Load/Store (PUSH/POP and LDM/STM)
    ThumbDecoder {
        format: 0b1011_0100_0000_0000,
        mask: 0b1111_0110_0000_0000,
        executor: CPU::thumb_push_pop_registers,
    },
    ThumbDecoder {
        format: 0b1100_0000_0000_0000,
        mask: 0b1111_0000_0000_0000,
        executor: CPU::thumb_multiple_load_store,
    },
    // Jumps and Calls
    ThumbDecoder {
        format: 0b1101_0000_0000_0000,
        mask: 0b1111_0000_0000_0000,
        executor: CPU::thumb_conditional_branch,
    },
    ThumbDecoder {
        format: 0b1110_0000_0000_0000,
        mask: 0b1111_1000_0000_0000,
        executor: CPU::thumb_unconditional_branch,
    },
    ThumbDecoder {
        format: 0b1111_0000_0000_0000,
        mask: 0b1111_1000_0000_0000,
        executor: CPU::thumb_long_branch_with_link_1,
    },
    ThumbDecoder {
        format: 0b1111_1000_0000_0000,
        mask: 0b1111_1000_0000_0000,
        executor: CPU::thumb_long_branch_with_link_2,
    },
    ThumbDecoder {
        format: 0b1101_1111_0000_0000,
        mask: 0b1111_1111_0000_0000,
        executor: CPU::thumb_software_interrupt,
    },
    ThumbDecoder {
        format: 0b1011_1110_0000_0000,
        mask: 0b1111_1111_0000_0000,
        executor: CPU::thumb_breakpoint,
    },
];
