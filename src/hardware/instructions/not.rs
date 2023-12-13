use std::process;

use crate::hardware::registers::Registers;

use super::update_flags;

/**
Bitwise NOT the values of two registers together and stores the result in a register.
*/
pub fn not(instruction: u16, registers: &mut Registers) {
    let destination_register = (instruction >> 9) & 0x7;
    let source_register = (instruction >> 6) & 0x7;
    let source_register_value = registers.read(source_register);

    registers.update(destination_register, !source_register_value);

    update_flags(destination_register, registers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_not() {
        let instruction = 0b1001_001_011_1_11111;
        let mut registers = Registers::initial();

        registers.update(0b011, 0b110001);

        not(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read(0b001), !0b110001);
    }
}
