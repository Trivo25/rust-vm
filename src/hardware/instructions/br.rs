use crate::hardware::registers::Registers;

use super::sign_extended;

/**

The branch opcode is 0000, which is the first 4 bits of the instruction.

||||||
|---|---|---|---|---|
| 4bit  | 1bit  | 1bit  | 1bit | 9bit  |
|  0000  | n  | z  | p | PCOffset9  |

The branch instruction is used to branch to a different location in memory if a certain condition is met.
n, z, and p are the condition flags, and PCOffset9 is the offset to branch to.
*/
pub fn br(instruction: u16, registers: &mut Registers) {
    let pc_offset = sign_extended(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;

    let current_flag = registers.read_cond_flag() as u16;

    if cond_flag == current_flag {
        registers.update_program_counter(registers.read_program_counter() + pc_offset);
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::instructions::conditional_flags::ConditionalFlags;

    use super::*;

    #[test]
    fn test_branch_negative_condition() {
        let instruction = 0b0000_1_0_0_000000011;

        let mut registers = Registers::initial();

        registers.update_cond_flag(ConditionalFlags::Negative);

        br(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read_program_counter(), 0b11 + 0x3000);
    }

    #[test]
    fn test_branch_positive_condition() {
        let instruction = 0b0000_0_0_1_000000011;

        let mut registers = Registers::initial();

        registers.update_cond_flag(ConditionalFlags::Positive);

        br(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read_program_counter(), 0b11 + 0x3000);
    }

    #[test]
    fn test_branch_zero_condition() {
        let instruction = 0b0000_0_1_0_000000011;

        let mut registers = Registers::initial();

        registers.update_cond_flag(ConditionalFlags::Zero);

        br(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read_program_counter(), 0b11 + 0x3000);
    }
}
