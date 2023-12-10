use std::usize;

const MEMORY_MAX: usize = 1 << 16;

type Memory = [u16; MEMORY_MAX];
enum Instructions {
    BR = 0,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

impl TryFrom<u16> for Instructions {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instructions::BR),
            1 => Ok(Instructions::ADD),
            2 => Ok(Instructions::LD),
            3 => Ok(Instructions::ST),
            4 => Ok(Instructions::JSR),
            5 => Ok(Instructions::AND),
            6 => Ok(Instructions::LDR),
            7 => Ok(Instructions::STR),
            8 => Ok(Instructions::RTI),
            9 => Ok(Instructions::NOT),
            10 => Ok(Instructions::LDI),
            11 => Ok(Instructions::STI),
            12 => Ok(Instructions::JMP),
            13 => Ok(Instructions::RES),
            14 => Ok(Instructions::LEA),
            15 => Ok(Instructions::TRAP),
            _ => Err(()),
        }
    }
}

pub enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    RProgramCounter, // Program Counter
    RConditional,    // Condition Flag
    RTotalCount,
}

impl From<Registers> for usize {
    fn from(r: Registers) -> Self {
        r as usize
    }
}
enum ConditionalFlags {
    Positive = 1 << 0, // P
    Zero = 1 << 1,     // Z
    Negative = 1 << 2, // N
}

pub fn vm(argc: i32, _argv: Vec<String>) {
    if argc < 2 {
        println!("lc3 [image-file1] ...");
        std::process::exit(2);
    }
    /*
    for arg in argv {
        println!("arg: {}", arg);
        if !read_image(&arg) {
            println!("failed to load image: {}", arg);
            std::process::exit(1);
        }
    } */

    let memory: Memory = [0; MEMORY_MAX];
    let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

    registers[Registers::RConditional as usize] = ConditionalFlags::Zero as u16;

    // initialize program counter to start position of the program
    // 0x3000 is the default

    registers[Registers::RProgramCounter as usize] = 0x3000;

    let is_running = true;

    while is_running {
        // fetch

        let instruction = memory[registers[Registers::RProgramCounter as usize] as usize];
        let op_code = instruction >> 12;

        let instruction_kind = Instructions::try_from(op_code);

        match instruction_kind {
            Ok(Instructions::ADD) => add(instruction, &mut registers),
            Ok(Instructions::AND) => {}
            Ok(Instructions::NOT) => {}
            Ok(Instructions::BR) => branch(instruction, &mut registers),
            Ok(Instructions::JMP) => jump(instruction, &mut registers),
            Ok(Instructions::JSR) => jump_register(instruction, &mut registers),
            Ok(Instructions::LD) => {}
            Ok(Instructions::LDI) => {}
            Ok(Instructions::LDR) => {}
            Ok(Instructions::LEA) => {}
            Ok(Instructions::ST) => {}
            Ok(Instructions::STI) => {}
            Ok(Instructions::STR) => {}
            Ok(Instructions::TRAP) => {}
            _ => {
                println!("bad opcode");
                std::process::exit(1);
            }
        }
    }
}

pub fn add(instruction: u16, registers: &mut [u16; 10]) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extended(instruction & 0x1F, 5);
        registers[r0 as usize] = registers[r1 as usize] + imm5;
    } else {
        let r2 = instruction & 0x7;
        registers[r0 as usize] = registers[r1 as usize] + registers[r2 as usize];
    }

    update_flags(r0, registers);
}

fn branch(instruction: u16, registers: &mut [u16; 10]) {
    let pc_offset = sign_extended(instruction & 0x1FF, 9);
    let conditional_flag = (instruction >> 9) & 0x7;

    if conditional_flag & registers[Registers::RConditional as usize] == 1 {
        registers[Registers::RProgramCounter as usize] += pc_offset;
    }
}

fn jump(instruction: u16, registers: &mut [u16; 10]) {
    let base_r = (instruction >> 6) & 0x7;
    registers[Registers::RProgramCounter as usize] = registers[base_r as usize];
}

fn jump_register(instruction: u16, registers: &mut [u16; 10]) {
    let long_flag = (instruction >> 11) & 1;

    registers[Registers::R7 as usize] = registers[Registers::RProgramCounter as usize];

    if long_flag == 1 {
        let offset = sign_extended(instruction & 0x7FF, 11);
        registers[Registers::RProgramCounter as usize] += offset;
    } else {
        let r1 = (instruction >> 6) & 0x7;
        registers[Registers::RProgramCounter as usize] = registers[r1 as usize];
    }
}

fn sign_extended(x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

fn update_flags(r: u16, registers: &mut [u16; 10]) {
    if registers[r as usize] == 0 {
        registers[Registers::RConditional as usize] = ConditionalFlags::Zero as u16;
    } else if registers[r as usize] >> 15 == 1 {
        registers[Registers::RConditional as usize] = ConditionalFlags::Negative as u16;
    } else {
        registers[Registers::RConditional as usize] = ConditionalFlags::Positive as u16;
    }
}

pub fn print_registers(registers: &[u16; 10]) {
    for i in 0..(Registers::RTotalCount as usize) {
        if i == Registers::RProgramCounter as usize {
            println!("PC: {}", registers[i]);
            continue;
        } else if i == Registers::RConditional as usize {
            println!("CC: {}", registers[i]);
            continue;
        } else {
            println!("R{}: {}", i, registers[i]);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEBUG_PRINT: bool = false;

    #[test]
    fn test_add() {
        let instruction = 0b0001_000_001_0_00_010;

        let a = 3;
        let b = 5;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

        registers[Registers::R1 as usize] = a;
        registers[Registers::R2 as usize] = b;

        add(instruction, &mut registers);

        if DEBUG_PRINT {
            print_registers(&registers);
        }
        assert_eq!(registers[Registers::R0 as usize], a + b);
    }

    #[test]
    fn test_jump() {
        let instruction = 0b1100_000_001_000000;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

        registers[Registers::RProgramCounter as usize] = 0;
        registers[Registers::R1 as usize] = 5;

        jump(instruction, &mut registers);

        if DEBUG_PRINT {
            print_registers(&registers);
        }
        assert_eq!(registers[Registers::RProgramCounter as usize], 5);
    }
    #[test]
    fn test_jump_register_jsr() {
        let instruction = 0b0100_1_00000001000;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

        registers[Registers::RProgramCounter as usize] = 10;

        jump_register(instruction, &mut registers);

        if DEBUG_PRINT {
            print_registers(&registers);
        }

        assert_eq!(registers[Registers::RProgramCounter as usize], 18);
    }

    #[test]
    fn test_jump_register_jsrr() {
        let instruction = 0b0100_0_00_100_000000;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

        registers[Registers::RProgramCounter as usize] = 10;

        registers[Registers::R4 as usize] = 31;

        jump_register(instruction, &mut registers);

        if DEBUG_PRINT {
            print_registers(&registers);
        }
        assert_eq!(registers[Registers::RProgramCounter as usize], 31);
    }
}
