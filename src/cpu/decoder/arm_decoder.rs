pub fn decode_arm(opcode: u32) -> ArmInstruction {
    for format in ARM_INSTRUCTION_FORMATS {
        if format.matches(opcode) {
            return format.instruction;
        }
    }

    ArmInstruction::Unimplemented
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
struct ArmInstructionFormatter {
    format: u32,
    mask: u32,
    instruction: ArmInstruction,
}

impl ArmInstructionFormatter {
    fn matches(&self, opcode: u32) -> bool {
        (opcode & self.mask) == self.format
    }
}

const ARM_INSTRUCTION_FORMATS: [ArmInstructionFormatter; 14 ]= [
        ArmInstructionFormatter {
            format: 0b0000_0001_0010_1111_1111_1111_0001_0000,
            mask: 0b0000_1111_1111_1111_1111_1111_1111_0000,
            instruction: ArmInstruction::BranchAndBranchExchange,
        },
        ArmInstructionFormatter {
            format: 0b0000_1000_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
            instruction: ArmInstruction::BlockDataTransfer,
        },
        ArmInstructionFormatter {
            format: 0b0000_1010_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1110_0000_0000_0000_0000_0000_0000,
            instruction: ArmInstruction::Branch,
        },
        ArmInstructionFormatter {
            format: 0b0000_1111_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1111_0000_0000_0000_0000_0000_0000,
            instruction: ArmInstruction::SoftwareInterrupt,
        },
        ArmInstructionFormatter {
            format: 0b0000_0110_0000_0000_0000_0000_0001_0000,
            mask: 0b0000_1110_0000_0000_0000_0000_0001_0000,
            instruction: ArmInstruction::Undefined,
        },
        ArmInstructionFormatter {
            format: 0b0000_0100_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1100_0000_0000_0000_0000_0000_0000,
            instruction: ArmInstruction::SingleDataTransfer,
        },
        ArmInstructionFormatter {
            format: 0b0000_0001_0000_0000_0000_0000_1001_0000,
            mask: 0b0000_1111_1000_0000_0000_1111_1111_0000,
            instruction: ArmInstruction::SingleDataSwap,
        },
        ArmInstructionFormatter {
            format: 0b0000_0000_0000_0000_0000_0000_1001_0000,
            mask: 0b0000_1111_1000_0000_0000_0000_1111_0000,
            instruction: ArmInstruction::Multiply,
        },
        ArmInstructionFormatter {
            format: 0b0000_0000_1000_0000_0000_0000_1001_0000,
            mask: 0b0000_1111_1000_0000_0000_0000_1111_0000,
            instruction: ArmInstruction::MultiplyLong,
        },
        ArmInstructionFormatter {
            format: 0b0000_0000_0000_0000_0000_0000_1001_0000,
            mask: 0b0000_1110_0100_0000_0000_1111_1001_0000,
            instruction: ArmInstruction::HalfwordDataTransferRegister,
        },
        ArmInstructionFormatter {
            format: 0b0000_0000_0100_0000_0000_0000_1001_0000,
            mask: 0b0000_1110_0100_0000_0000_0000_1001_0000,
            instruction: ArmInstruction::HalfwordDataTransferImmediate,
        },
        ArmInstructionFormatter {
            format: 0b0000_0001_0000_1111_0000_0000_0000_0000,
            mask: 0b0000_1111_1011_1111_0000_0000_0000_0000,
            instruction: ArmInstruction::PsrTransferMrs,
        },
        ArmInstructionFormatter {
            format: 0b0000_0001_0010_0000_1111_0000_0000_0000,
            mask: 0b0000_1101_1011_0000_1111_0000_0000_0000,
            instruction: ArmInstruction::PsrTransferMsr,
        },
        ArmInstructionFormatter {
            format: 0b0000_0000_0000_0000_0000_0000_0000_0000,
            mask: 0b0000_1100_0000_0000_0000_0000_0000_0000,
            instruction: ArmInstruction::DataProcessing,
        },
    ];

