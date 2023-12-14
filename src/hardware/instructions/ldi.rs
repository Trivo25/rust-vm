use crate::hardware::{memory::Memory, registers::Registers};

use super::{sign_extended, update_flags};

/**
Load value from an address stored in memory into a register.
*/
pub fn ldi(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let destination_register = (instruction >> 9) & 0x7;
    let offset = sign_extended(instruction & 0x1FF, 9);

    let address = memory.read(offset + registers.read_program_counter());

    registers.update(destination_register, memory.read(address));
    update_flags(destination_register, registers)
}

#[cfg(test)]
mod tests {

    use crate::hardware::memory;

    use super::*;

    #[test]
    fn test_load_registers() {
        let instruction = 0b1010_001_000000001;

        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        let pc = registers.read_program_counter();

        memory.write(sign_extended(0b000000001, 9) + pc, 0x12);
        memory.write(0x12, 0x51);
        ldi(instruction, &mut registers, &mut memory);

        registers.pretty_print();

        assert_eq!(registers.read(0b001), 0x51);
    }
}
