use super::CPUInstruction;

pub fn decode(opcode: u32) -> CPUInstruction {
    if is_branch_and_branch_exchange(opcode)
    if is_block_data_transfer(opcode)            
    if is_branch(opcode)                          
    if is_software_interrupt(opcode)              
    if is_undefined(opcode)                       
    if is_single_data_transfer(opcode)            
    if is_single_data_swap(opcode)                
    if is_multiply(opcode)                        
    if is_multiply_long(opcode)                   
    if is_halfword_data_transfer_register(opcode) 
    if is_halfword_data_transfer_immediate(opcode)
    if is_psr_transfer_mrs(opcode)                
    if is_psr_transfer_msr(opcode)                
    if is_data_processing(opcode)   

    CPUInstruction::ARMUnimplemented;
}

macro_rules! arm_instr_check {
    ($( $fn_name:ident => ($format:expr, $mask:expr) ),* $(,)?) => {
        $(  
            fn $fn_name(opcode: u32) -> bool {
                (opcode & $mask) == $format
            }
        )*
    };
}

arm_instr_check!(
    is_branch_and_branch_exchange       => (0b0000_0001_0010_1111_1111_1111_0001_0000, 0b0000_1111_1111_1111_1111_1111_1111_0000),
    is_block_data_transfer              => (0b0000_1000_0000_0000_0000_0000_0000_0000, 0b0000_1110_0000_0000_0000_0000_0000_0000),
    is_branch                           => (0b0000_1010_0000_0000_0000_0000_0000_0000, 0b0000_1110_0000_0000_0000_0000_0000_0000),
    is_software_interrupt               => (0b0000_1111_0000_0000_0000_0000_0000_0000, 0b0000_1111_0000_0000_0000_0000_0000_0000),
    is_undefined                        => (0b0000_0110_0000_0000_0000_0000_0001_0000, 0b0000_1110_0000_0000_0000_0000_0001_0000),
    is_single_data_transfer             => (0b0000_0100_0000_0000_0000_0000_0000_0000, 0b0000_1100_0000_0000_0000_0000_0000_0000),
    is_single_data_swap                 => (0b0000_0001_0000_0000_0000_0000_1001_0000, 0b0000_1111_1000_0000_0000_1111_1111_0000),
    is_multiply                         => (0b0000_0000_0000_0000_0000_0000_1001_0000, 0b0000_1111_1000_0000_0000_0000_1111_0000),
    is_multiply_long                    => (0b0000_0000_1000_0000_0000_0000_1001_0000, 0b0000_1111_1000_0000_0000_0000_1111_0000),
    is_halfword_data_transfer_register  => (0b0000_0000_0000_0000_0000_0000_1001_0000, 0b0000_1110_0100_0000_0000_1111_1001_0000),
    is_halfword_data_transfer_immediate => (0b0000_0000_0100_0000_0000_0000_1001_0000, 0b0000_1110_0100_0000_0000_0000_1001_0000),
    is_psr_transfer_mrs                 => (0b0000_0001_0000_1111_0000_0000_0000_0000, 0b0000_1111_1011_1111_0000_0000_0000_0000),
    is_psr_transfer_msr                 => (0b0000_0001_0010_0000_1111_0000_0000_0000, 0b0000_1101_1011_0000_1111_0000_0000_0000),
    is_data_processing                  => (0b0000_0000_0000_0000_0000_0000_0000_0000, 0b0000_1100_0000_0000_0000_0000_0000_0000),
);
