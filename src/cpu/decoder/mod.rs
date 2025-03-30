mod thumb_decoder;
mod arm_decoder;

use thumb_decoder::decode as thumb_decode;
use arm_decoder::decode as arm_decode;

#[derive(Debug, Copy, Clone, PartialEq)] 
pub enum CPUInstruction {
    ThumbSoftwareInterrupt { user_data: u8 },
    ThumbUnconditionalBranch,
    ThumbConditionalBranch,
    ThumbMultipleLoadstore,
    ThumbLongBranchWithLink,
    ThumbAddOffsetToStackPointer,
    ThumbPushPopRegisters,
    ThumbLoadStoreHalfword,
    ThumbSPRelativeLoadStore,
    ThumbLoadAddress,
    ThumbLoadStoreWithImmediateOffset,
    ThumbLoadStoreWithRegisterOffset,
    ThumbLoadStoreSignExtendedByteHalfword,
    ThumbPCRelativeLoad,
    ThumbHiRegisterOperationsBranchExchange,
    ThumbALUOperations { op: u8, rs: u8, rd: u8},
    ThumbMoveCompareAddSubtractImmediate,
    ThumbAddSubtract,
    ThumbMoveShiftedRegister,
    ThumbUnimplemented,

    ARMBranchAndBranchExchange,
    ARMBlockDataTransfer,
    ARMBranch,
    ARMSoftwareInterrupt,
    ARMUndefined,
    ARMSingleDataTransfer,
    ARMSingleDataSwap,
    ARMMultiply,
    ARMMultiplyLong,
    ARMHalfwordDataTransferRegister,
    ARMHalfwordDataTransferImmediate,
    ARMPSRTransferMRS,
    ARMPSRTransferMSR,
    ARMDataProcessing,
    ARMUnimplemented,
}

pub fn decode(opcode: u32, is_in_thumb_mode: bool) -> CPUInstruction {
    if is_in_thumb_mode {
        // Will this work?
        thumb_decode(opcode as u16)
    } else {
        arm_decode(opcode)
    }
}