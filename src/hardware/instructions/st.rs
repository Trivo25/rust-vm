use crate::hardware::{memory::Memory, registers::Registers};

use super::{add, sign_extended, update_flags};

/**
Store content of register into an address in memory specified by the offset and program counter.
*/
pub fn st(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let source_register = (instruction >> 9) & 0x7;
    let pc = registers.read_program_counter();
    let offset = sign_extended(instruction & 0x1FF, 9);

    let address: u16 = pc + offset;
    let value = registers.read(source_register);

    memory.write(address, value);
}

#[cfg(test)]
mod tests {

    use crate::hardware::memory;

    use super::*;

    #[test]
    fn test_load_registers() {
        let instruction = 0b0011_010_000110101;

        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        registers.update(0b010, 0x5);

        st(instruction, &mut registers, &mut memory);

        registers.pretty_print();

        assert_eq!(
            memory.read(sign_extended(0b000110101, 9) + registers.read_program_counter()),
            0x5
        );
    }
}
