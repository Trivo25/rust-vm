use super::{sign_extended, update_flags};
use crate::hardware::registers::Registers;

pub fn add(instruction: u16, registers: &mut Registers) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extended(instruction & 0x1F, 5);
        registers.update(r0, registers.read(r1) + imm5);
    } else {
        let r2 = instruction & 0x7;
        registers.update(r0, registers.read(r1) + registers.read(r2));
    }

    update_flags(r0, registers);
}
