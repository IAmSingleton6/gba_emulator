use crate::cpu::ArmExecutor;

pub fn decode_arm(opcode: u32) -> ArmExecutor {
    for format in ARM_INSTRUCTION_FORMATS {
        if format.matches(opcode) {
            return format.executor;
        }
    }

    ArmInstruction::no_op
}

#[derive(Debug)]
pub enum ArmInstruction {
    BranchAndBranchExchange,
    BlockDataTransfer,
    Branch,
    SoftwareInterrupt,
    Undefined,
    SingleDataTransfer,
    SingleDataSwap,
    Multiply,
    MultiplyLong,
    HalfwordDataTransferRegister,
    HalfwordDataTransferImmediate,
    PsrTransferMrs,
    PsrTransferMsr,
    DataProcessing,
    Unimplemented,
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

const ARM_INSTRUCTION_FORMATS: [ArmDecoder; 14 ]= [
        ArmDecoder {
            format: 0b0000_0001_0010_1111_1111_1111_0001_0000,
            mask: 0b0000_1111_1111_1111_1111_1111_1111_0000,
            executor: ArmInstruction::BranchAndBranchExchange,
        },
        ArmDecoder {
            format: 0b0000_1000_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
            executor: ArmInstruction::BlockDataTransfer,
        },
        ArmDecoder {
            format: 0b0000_1010_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
            executor: ArmInstruction::Branch,
        },
        ArmDecoder {
            format: 0b0000_1111_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1111_0000_0000_0000_0000_0000_0000,
            executor: ArmInstruction::SoftwareInterrupt,
        },
        ArmDecoder {
            format: 0b0000_0110_0000_0000_0000_0000_0001_0000,
            mask: 0b0000_1110_0000_0000_0000_0000_0001_0000,
            executor: ArmInstruction::Undefined,
        },
        ArmDecoder {
            format: 0b0000_0100_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1100_0000_0000_0000_0000_0000_0000,
            executor: ArmInstruction::SingleDataTransfer,
        },
        ArmDecoder {
            format: 0b0000_0001_0000_0000_0000_0000_1001_0000,
            mask: 0b0000_1111_1000_0000_0000_1111_1111_0000,
            executor: ArmInstruction::SingleDataSwap,
        },
        ArmDecoder {
            format: 0b0000_0000_0000_0000_0000_0000_1001_0000,
            mask: 0b0000_1111_1000_0000_0000_0000_1111_0000,
            executor: ArmInstruction::Multiply,
        },
        ArmDecoder {
            format: 0b0000_0000_1000_0000_0000_0000_1001_0000,
            mask: 0b0000_1111_1000_0000_0000_0000_1111_0000,
            executor: ArmInstruction::MultiplyLong,
        },
        ArmDecoder {
            format: 0b0000_0000_0000_0000_0000_0000_1001_0000,
            mask: 0b0000_1110_0100_0000_0000_1111_1001_0000,
            executor: ArmInstruction::HalfwordDataTransferRegister,
        },
        ArmDecoder {
            format: 0b0000_0000_0100_0000_0000_0000_1001_0000,
            mask: 0b0000_1110_0100_0000_0000_0000_1001_0000,
            executor: ArmInstruction::HalfwordDataTransferImmediate,
        },
        ArmDecoder {
            format: 0b0000_0001_0000_1111_0000_0000_0000_0000,
            mask: 0b0000_1111_1011_1111_0000_0000_0000_0000,
            executor: ArmInstruction::PsrTransferMrs,
        },
        ArmDecoder {
            format: 0b0000_0001_0010_0000_1111_0000_0000_0000,
            mask: 0b0000_1101_1011_0000_1111_0000_0000_0000,
            executor: ArmInstruction::PsrTransferMsr,
        },
        ArmDecoder {
            format: 0b0000_0000_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1100_0000_0000_0000_0000_0000_0000,
            executor: ArmInstruction::DataProcessing,
        },
    ];

