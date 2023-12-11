use std::process;

use crate::hardware::registers::Registers;

use super::{sign_extended, update_flags};

/**
Bitwise ANDs the values of two registers together and stores the result in a register.
If in immediate mode, ANDs the value of a register and a sign-extended immediate value together and stores the result in a register.
*/
pub fn and(instruction: u16, registers: &mut Registers) {
    let immediate_flag = (instruction >> 5) & 0x1;

    let destination_register = (instruction >> 9) & 0x7;
    let source_1: u16 = (instruction >> 6) & 0x7;
    let source_1_value = registers.read(source_1);

    if immediate_flag == 1 {
        let source_2: u16 = instruction & 0x7;

        let source_2_value = registers.read(source_2);

        registers.update(destination_register, source_1_value & source_2_value)
    } else {
        let immediate_value_extended = sign_extended(instruction & 0x1F, 5);
        registers.update(
            destination_register,
            source_1_value & immediate_value_extended,
        )
    }

    update_flags(destination_register, registers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_and() {
        let instruction = 0b0101_000_001_0_00_100;

        let mut registers = Registers::initial();

        registers.update(0b001, 4);
        registers.update(0b100, 5);

        and(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read(0x000), 0b101 & sign_extended(5, 5));
    }
}
