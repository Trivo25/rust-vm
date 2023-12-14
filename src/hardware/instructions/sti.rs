use crate::hardware::{memory::Memory, registers::Registers};

use super::{add, sign_extended, update_flags};

/**
Store content of register into an address in memory specified by the address thats in memory at the offset and program counter.
*/
pub fn sti(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let source_register = (instruction >> 9) & 0x7;
    let pc = registers.read_program_counter();
    let offset = sign_extended(instruction & 0x1FF, 9);

    let address: u16 = memory.read((pc as u32 + offset as u32) as u16);
    let value = registers.read(source_register);

    memory.write(address, value);
}

#[cfg(test)]
mod tests {

    use crate::hardware::memory;

    use super::*;

    #[test]
    fn test_sti() {
        let instruction = 0b1011_010_000110101;

        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        registers.update(0b010, 0x5);

        memory.write(registers.read_program_counter() + 0b000110101, 0x100);

        sti(instruction, &mut registers, &mut memory);

        registers.pretty_print();

        assert_eq!(memory.read(0x100), 0x5);
    }
}
