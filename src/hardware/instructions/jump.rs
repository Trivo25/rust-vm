use crate::hardware::registers::Registers;

use super::sign_extended;

/**
Jump to a location in memory.
*/
pub fn jump(instruction: u16, registers: &mut Registers) {
    let base_register = (instruction >> 6) & 0x7;
    let base_register_value = registers.read(base_register);

    registers.update_program_counter(base_register_value);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_jump() {
        let instruction = 0b1100_000_001_000000;

        let mut registers = Registers::initial();

        registers.update(0b001, 0x555);

        jump(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read_program_counter(), 0x555);
    }
}
