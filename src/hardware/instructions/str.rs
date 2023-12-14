use crate::hardware::{memory::Memory, registers::Registers};

use super::sign_extended;

/**
Store content of register into an address in memory specified by the address thats in memory at the offset and program counter.
*/
pub fn str(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let source_register = (instruction >> 9) & 0x7;
    let base_register = (instruction >> 6) & 0x7;

    let offset = sign_extended(instruction & 0x3F, 6);

    let address: u16 = registers.read(base_register) + offset;
    let value = registers.read(source_register);

    memory.write(address, value);
}

#[cfg(test)]
mod tests {

    use crate::hardware::memory;

    use super::*;

    #[test]
    fn test_str() {
        let instruction = 0b0111_010_000_110101;

        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        registers.update(0b010, 0x5);
        registers.update(0b000, 0x1);

        str(instruction, &mut registers, &mut memory);

        registers.pretty_print();

        assert_eq!(memory.read(sign_extended(0b110101, 6) + 0x1), 0x5);
    }
}
