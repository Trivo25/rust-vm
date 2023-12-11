use crate::hardware::registers::Registers;

use super::sign_extended;

pub fn branch(instruction: u16, registers: &mut Registers) {
    let pc_offset = sign_extended(instruction & 0x1FF, 9);
    let conditional_flag = (instruction >> 9) & 0x7;

    if conditional_flag & registers.read_cond_flag() as u16 == 1 {
        registers.update_program_counter(registers.read_program_counter() + pc_offset);
    }
}
