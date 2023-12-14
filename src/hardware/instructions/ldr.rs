use crate::hardware::{memory::Memory, registers::Registers};

use super::{add, sign_extended, update_flags};

/**
Load from memory into a register from address in BaseR + offset
*/
pub fn ldr(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let destination_register = (instruction >> 9) & 0x7;
    let base_register = (instruction >> 6) & 0x7;
    let offset = sign_extended(instruction & 0x3F, 6);

    let address = registers.read(base_register) as u32 + offset as u32;

    registers.update(destination_register, memory.read(address as u16));
    update_flags(destination_register, registers)
}

#[cfg(test)]
mod tests {

    use crate::hardware::memory;

    use super::*;

    #[test]
    fn test_load_registers() {
        let instruction = 0b0110_001_010_110101;

        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        registers.update(0b010, 0x5);

        memory.write(sign_extended(0b110101, 6) + 0x5, 0x55);

        ldr(instruction, &mut registers, &mut memory);

        registers.pretty_print();

        assert_eq!(registers.read(0b001), 0x55);
    }
}
