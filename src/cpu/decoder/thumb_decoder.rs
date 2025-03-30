use super::CPUInstruction;

pub fn decode(opcode: u16) -> CPUInstruction {
    if is_software_interrupt(opcode) {
        return CPUInstruction::ThumbSoftwareInterrupt;
    }
    if is_unconditional_branch(opcode)
    if is_conditional_branch(opcode)
    if is_multiple_loadstore(opcode)
    if is_long_branch_with_link(opcode)
    if is_add_offset_to_stack_pointer(opcode)
    if is_push_pop_registers(opcode)
    if is_load_store_halfword(opcode)
    if is_sp_relative_load_store(opcode)
    if is_load_address(opcode)
    if is_load_store_with_immediate_offset(opcode)
    if is_load_store_with_register_offset(opcode)
    if is_load_store_sign_extended_byte_halfword(opcode)
    if is_pc_relative_load(opcode)
    if is_hi_register_operations_branch_exchange(opcode)
    if is_alu_operations(opcode)
    if is_move_compare_add_subtract_immediate(opcode)
    if is_add_subtract(opcode)
    if is_move_shifted_register(opcode)

    CPUInstruction::ThumbUnimplemented
}

macro_rules! thumb_instr_check {
    ($( $fn_name:ident => ($format:expr, $mask:expr) ),* $(,)?) => {
        $(  
            fn $fn_name(opcode: u16) -> bool {
                (opcode & $mask) == $format
            }
        )*
    };
}

thumb_instr_check! (
    is_software_interrupt                       => (0b1101_1111_0000_0000, 0b1111_1111_0000_0000),
    is_unconditional_branch                     => (0b1110_0000_0000_0000, 0b1111_1000_0000_0000),
    is_conditional_branch                       => (0b1101_0000_0000_0000, 0b1111_0000_0000_0000),
    is_multiple_loadstore                       => (0b1100_0000_0000_0000, 0b1111_0000_0000_0000),
    is_long_branch_with_link                    => (0b1111_0000_0000_0000, 0b1111_0000_0000_0000),
    is_add_offset_to_stack_pointer              => (0b1011_0000_0000_0000, 0b1111_1111_0000_0000),
    is_push_pop_registers                       => (0b1011_0100_0000_0000, 0b1111_0110_0000_0000),
    is_load_store_halfword                      => (0b1000_0000_0000_0000, 0b1111_0000_0000_0000),
    is_sp_relative_load_store                   => (0b1001_0000_0000_0000, 0b1111_0000_0000_0000),
    is_load_address                             => (0b1010_0000_0000_0000, 0b1111_0000_0000_0000),
    is_load_store_with_immediate_offset         => (0b0110_0000_0000_0000, 0b1110_0000_0000_0000),
    is_load_store_with_register_offset          => (0b0101_0000_0000_0000, 0b1111_0010_0000_0000),
    is_load_store_sign_extended_byte_halfword   => (0b0101_0010_0000_0000, 0b1111_0010_0000_0000),
    is_pc_relative_load                         => (0b0100_1000_0000_0000, 0b1111_1000_0000_0000),
    is_hi_register_operations_branch_exchange   => (0b0100_0100_0000_0000, 0b1111_1100_0000_0000),
    is_alu_operations                           => (0b0100_0000_0000_0000, 0b1111_1100_0000_0000),
    is_move_compare_add_subtract_immediate      => (0b1110_0000_0000_0000, 0b0010_0000_0000_0000),
    is_add_subtract                             => (0b0001_1000_0000_0000, 0b1111_1000_0000_0000),
    is_move_shifted_register                    => (0b0000_0000_0000_0000, 0b1110_0000_0000_0000),
);