use crate::hardware::{memory::Memory, registers::Registers};

use super::{add, sign_extended, update_flags};

/**
Load from memory into a register
*/
pub fn ld(instruction: u16, registers: &mut Registers, memory: &Memory) {
    let destination_register = (instruction >> 9) & 0x7;
    let offset = sign_extended(instruction & 0x1FF, 9);
    let pc = registers.read_program_counter();

    let address = pc + offset;

    registers.update(destination_register, memory.read(address));

    update_flags(destination_register, registers)
}

#[cfg(test)]
mod tests {

    use crate::hardware::memory;

    use super::*;

    #[test]
    fn test_load() {
        let instruction = 0b0010_000_000110101;

        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        memory.write(registers.read_program_counter() + 0b000110101, 0x1234);

        ld(instruction, &mut registers, &memory);

        registers.pretty_print();

        assert_eq!(registers.read(0b000), 0x1234);
    }
}
