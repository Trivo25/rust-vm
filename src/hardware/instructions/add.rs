use super::{sign_extended, update_flags};
use crate::hardware::registers::Registers;

/**

The add opcode consists of two branches: add and add immediate, which are differentiated by the 5th bit.

#### Add:

The first branch is the add branch, which adds the values of two registers together and stores the result in a register.


|||||||
|---|---|---|---|---|---|
| 4bit  | 3bit  | 3bit  | 1bit|2bit  | 3bit  |
|  0001  | DR  | SR1  | 0  | 00 | SR2  |


The second branch is the add immediate branch, which adds the value of a register and a sign-extended immediate value together and stores the result in a register.


||||||
|---|---|---|---|---|
| 4bit  | 3bit  | 3bit  | 1bit  | 5bit  |
|  0001  | DR  | SR1  | 1  | imm5  |
*/
pub fn add(instruction: u16, registers: &mut Registers) {
    let destination_register = (instruction >> 9) & 0x7;
    let source_1: u16 = (instruction >> 6) & 0x7;

    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extended(instruction & 0x1F, 5);
        registers.update(destination_register, registers.read(source_1) + imm5);
    } else {
        let source_2: u16 = instruction & 0x7;
        registers.update(
            destination_register,
            registers.read(source_1) + registers.read(source_2),
        );
    }

    update_flags(destination_register, registers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let instruction = 0b0001_000_001_0_00_010;

        let a = 0x3;
        let b = 0x7;

        let mut registers = Registers::initial();

        registers.update(1, a);
        registers.update(2, b);

        add(instruction, &mut registers);

        registers.pretty_print();

        assert_eq!(registers.read(0), a + b);
    }
}
