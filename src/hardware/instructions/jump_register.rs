use crate::hardware::registers::Registers;

use super::sign_extended;

/**
Jump to a location in memory based on the value of a register.
*/
pub fn jump_register(instruction: u16, registers: &mut Registers) {
    let long_flag = (instruction >> 11) & 0x1;
    registers.update(7, registers.read_program_counter());

    if long_flag == 1 {
        // jsr
        let offset = sign_extended(instruction & 0x7FF, 11);
        registers.update_program_counter(registers.read_program_counter() + offset);
    } else {
        // jsrr
        let base_register = (instruction >> 6) & 0x7;
        registers.update_program_counter(registers.read(base_register));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_jump_register() {
        let instruction = 0b0100_1_00000000011;

        let mut registers = Registers::initial();

        jump_register(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read_program_counter(), 0x3003);
    }

    #[test]
    fn test_jump_register_jsrr() {
        let instruction = 0b0100_0_00_010_000000;

        let mut registers = Registers::initial();

        registers.update(0b010, 1234);

        jump_register(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read_program_counter(), 1234);
    }
}
