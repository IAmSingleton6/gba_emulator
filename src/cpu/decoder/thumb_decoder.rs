pub fn decode_thumb(opcode: u16) -> ThumbInstruction {
    for format in THUMB_INSTRUCTION_FORMATS {
        if format.matches(opcode) {
            return format.instruction;
        }
    }

    ThumbInstruction::Unimplemented
}

#[derive(Debug)]
pub enum ThumbInstruction {
    SoftwareInterrupt,
    UnconditionalBranch,
    ConditionalBranch,
    MultipleLoadStore,
    LongBranchWithLink,
    AddOffsetToStackPointer,
    PushPopRegisters,
    LoadStoreHalfword,
    SpRelativeLoadStore,
    LoadAddress,
    LoadStoreWithImmediateOffset,
    LoadStoreWithRegisterOffset,
    LoadStoreSignExtendedByteHalfword,
    PcRelativeLoad,
    HiRegisterOperationsBranchExchange,
    AluOperations,
    MoveCompareAddSubtractImmediate,
    AddSubtract,
    MoveShiftedRegister,
    Unimplemented,
}

#[derive(Debug)]
struct ThumbInstructionFormatter {
    format: u16,
    mask: u16,
    instruction: ThumbInstruction,
}

impl ThumbInstructionFormatter {
    fn matches(&self, opcode: u16) -> bool {
        (opcode & self.mask) == self.format
    }
}

    const THUMB_INSTRUCTION_FORMATS: [ThumbInstructionFormatter; 19] = [
        ThumbInstructionFormatter {
            format: 0b1101_1111_0000_0000,
            mask: 0b1111_1111_0000_0000,
            instruction: ThumbInstruction::SoftwareInterrupt,
        },
        ThumbInstructionFormatter {
            format: 0b1110_0000_0000_0000,
            mask: 0b1111_1000_0000_0000,
            instruction: ThumbInstruction::UnconditionalBranch,
        },
        ThumbInstructionFormatter {
            format: 0b1101_0000_0000_0000,
            mask: 0b1111_0000_0000_0000,
            instruction: ThumbInstruction::ConditionalBranch,
        },
        ThumbInstructionFormatter {
            format: 0b1100_0000_0000_0000,
            mask: 0b1111_0000_0000_0000,
            instruction: ThumbInstruction::MultipleLoadStore,
        },
        ThumbInstructionFormatter {
            format: 0b1111_0000_0000_0000,
            mask: 0b1111_0000_0000_0000,
            instruction: ThumbInstruction::LongBranchWithLink,
        },
        ThumbInstructionFormatter {
            format: 0b1011_0000_0000_0000,
            mask: 0b1111_1111_0000_0000,
            instruction: ThumbInstruction::AddOffsetToStackPointer,
        },
        ThumbInstructionFormatter {
            format: 0b1011_0100_0000_0000,
            mask: 0b1111_0110_0000_0000,
            instruction: ThumbInstruction::PushPopRegisters,
        },
        ThumbInstructionFormatter {
            format: 0b1000_0000_0000_0000,
            mask: 0b1111_0000_0000_0000,
            instruction: ThumbInstruction::LoadStoreHalfword,
        },
        ThumbInstructionFormatter {
            format: 0b1001_0000_0000_0000,
            mask: 0b1111_0000_0000_0000,
            instruction: ThumbInstruction::SpRelativeLoadStore,
        },
        ThumbInstructionFormatter {
            format: 0b1010_0000_0000_0000,
            mask: 0b1111_0000_0000_0000,
            instruction: ThumbInstruction::LoadAddress,
        },
        ThumbInstructionFormatter {
            format: 0b0110_0000_0000_0000,
            mask: 0b1110_0000_0000_0000,
            instruction: ThumbInstruction::LoadStoreWithImmediateOffset,
        },
        ThumbInstructionFormatter {
            format: 0b0101_0000_0000_0000,
            mask: 0b1111_0010_0000_0000,
            instruction: ThumbInstruction::LoadStoreWithRegisterOffset,
        },
        ThumbInstructionFormatter {
            format: 0b0101_0010_0000_0000,
            mask: 0b1111_0010_0000_0000,
            instruction: ThumbInstruction::LoadStoreSignExtendedByteHalfword,
        },
        ThumbInstructionFormatter {
            format: 0b0100_1000_0000_0000,
            mask: 0b1111_1000_0000_0000,
            instruction: ThumbInstruction::PcRelativeLoad,
        },
        ThumbInstructionFormatter {
            format: 0b0100_0100_0000_0000,
            mask: 0b1111_1100_0000_0000,
            instruction: ThumbInstruction::HiRegisterOperationsBranchExchange,
        },
        ThumbInstructionFormatter {
            format: 0b0100_0000_0000_0000,
            mask: 0b1111_1100_0000_0000,
            instruction: ThumbInstruction::AluOperations,
        },
        ThumbInstructionFormatter {
            format: 0b1110_0000_0000_0000,
            mask: 0b0010_0000_0000_0000,
            instruction: ThumbInstruction::MoveCompareAddSubtractImmediate,
        },
        ThumbInstructionFormatter {
            format: 0b0001_1000_0000_0000,
            mask: 0b1111_1000_0000_0000,
            instruction: ThumbInstruction::AddSubtract,
        },
        ThumbInstructionFormatter {
            format: 0b0000_0000_0000_0000,
            mask: 0b1110_0000_0000_0000,
            instruction: ThumbInstruction::MoveShiftedRegister,
        },
    ];