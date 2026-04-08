use crate::cpu::ArmExecutor;
use crate::cpu::CPU;

pub fn decode_arm(opcode: u32) -> ArmExecutor {
    for (i, format) in ARM_INSTRUCTION_FORMATS.iter().enumerate() {
        let matches = format.matches(opcode);
        if matches {
            return format.executor;
        }
    }

    CPU::arm_no_op
}

#[derive(Debug)]
struct ArmDecoder {
    format: u32,
    mask: u32,
    executor: ArmExecutor,
}

impl ArmDecoder {
    fn matches(&self, opcode: u32) -> bool {
        (opcode & self.mask) == self.format
    }
}

const ARM_INSTRUCTION_FORMATS: [ArmDecoder; 15] = [
    ArmDecoder {
        format: 0b0000_0001_0010_1111_1111_1111_0001_0000,
        mask: 0b0000_1111_1111_1111_1111_1111_1111_0000,
        executor: CPU::arm_branch_and_branch_exchange,
    },
    ArmDecoder {
        format: 0b0000_1000_0000_0000_0000_0000_0000_0000,
        mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
        executor: CPU::arm_block_data_transfer,
    },
    ArmDecoder {
        format: 0b0000_1010_0000_0000_0000_0000_0000_0000,
        mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
        executor: CPU::arm_branch,
    },
    ArmDecoder {
        format: 0b0000_1111_0000_0000_0000_0000_0000_0000,
        mask: 0b0000_1111_0000_0000_0000_0000_0000_0000,
        executor: CPU::arm_software_interrupt,
    },
    ArmDecoder {
        format: 0b0000_0110_0000_0000_0000_0000_0001_0000,
        mask: 0b0000_1110_0000_0000_0000_0000_0001_0000,
        executor: CPU::arm_undefined,
    },
    // LDRH/STRH immediate offset variant (I=0)
    ArmDecoder {
        format: 0b0000_0000_0000_0000_0000_0000_1001_0000,
        mask: 0b0000_1110_0100_0000_0000_0000_1001_0000,
        executor: CPU::arm_halfword_data_transfer_immediate,
    },
    // LDRH/STRH register offset variant (I=1)
    ArmDecoder {
        format: 0b0000_0000_0100_0000_0000_0000_1001_0000,
        mask: 0b0000_1110_0100_0000_0000_1111_1001_0000,
        executor: CPU::arm_halfword_data_transfer_register,
    },
    ArmDecoder {
        format: 0b0000_0100_0000_0000_0000_0000_0000_0000,
        mask: 0b0000_1100_0000_0000_0000_0000_0000_0000,
        executor: CPU::arm_single_data_transfer,
    },
    ArmDecoder {
        format: 0b0000_0001_0000_0000_0000_0000_1001_0000,
        mask: 0b0000_1111_1000_0000_0000_1111_1111_0000,
        executor: CPU::arm_single_data_swap,
    },
    ArmDecoder {
        format: 0b0000_0000_0000_0000_0000_0000_1001_0000,
        mask: 0b0000_1111_1000_0000_0000_0000_1111_0000,
        executor: CPU::arm_multiply,
    },
    ArmDecoder {
        format: 0b0000_0000_1000_0000_0000_0000_1001_0000,
        mask: 0b0000_1111_1000_0000_0000_0000_1111_0000,
        executor: CPU::arm_multiply_long,
    },
    ArmDecoder {
        format: 0b0000_0001_0000_1111_0000_0000_0000_0000,
        mask: 0b0000_1111_1011_1111_0000_0000_0000_0000,
        executor: CPU::arm_psr_transfer_mrs,
    },
    ArmDecoder {
        format: 0b0000_0001_0010_0000_1111_0000_0000_0000,
        mask: 0b0000_1101_1011_0000_1111_0000_0000_0000,
        executor: CPU::arm_psr_transfer_msr,
    },
    ArmDecoder {
        format: 0b0000_0000_0000_0000_0000_0000_0000_0000,
        mask: 0b0000_1100_0000_0000_0000_0000_0000_0000,
        executor: CPU::arm_data_processing,
    },
    // Additional patterns for edge cases
    ArmDecoder {
        format: 0b0000_0000_0000_0000_0000_0000_0000_0000,
        mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
        executor: CPU::arm_data_processing,
    },
];
