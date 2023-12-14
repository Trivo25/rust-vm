use crate::hardware::{memory::Memory, registers::Registers};

use super::{sign_extended, update_flags};

/**
Load address into register calculated from PC + offset
*/
pub fn lea(instruction: u16, registers: &mut Registers) {
    let destination_register = (instruction >> 9) & 0x7;
    let offset = sign_extended(instruction & 0x1FF, 9);

    let address = (offset as u32) + (registers.read_program_counter() as u32);

    registers.update(destination_register, address as u16);
    update_flags(destination_register, registers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_effective() {
        let instruction = 0b1110_001_000000011;

        let mut registers = Registers::initial();

        let pc = registers.read_program_counter();

        lea(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read(0b001), pc + 0b000000011);
    }
}
