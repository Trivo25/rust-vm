use byteorder::{BigEndian, ReadBytesExt};

use std::{
    fs::File,
    io::{stdin, BufReader, Read},
};

const MEMORY_MAX: usize = 1 << 16;
type Memory = [u16; MEMORY_MAX];

enum TrapCodes {
    TrapGetC = 0x20,
    TrapOut = 0x21,
    TrapPuts = 0x22,
    TrapIn = 0x23,
    TrapPutsP = 0x24,
    TrapHalt = 0x25,
}

impl TryFrom<u16> for TrapCodes {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x20 => Ok(TrapCodes::TrapGetC),
            0x21 => Ok(TrapCodes::TrapOut),
            0x22 => Ok(TrapCodes::TrapPuts),
            0x23 => Ok(TrapCodes::TrapIn),
            0x24 => Ok(TrapCodes::TrapPutsP),
            0x25 => Ok(TrapCodes::TrapHalt),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
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

pub fn vm() {
    /*     if argc < 2 {
        println!("lc3 [image-file1] ...");
        std::process::exit(2);
    } */
    /*
    for arg in argv {
        println!("arg: {}", arg);
        if !read_image(&arg) {
            println!("failed to load image: {}", arg);
            std::process::exit(1);
        }
    } */

    let mut memory: Memory = [0; MEMORY_MAX];
    let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

    let f = File::open("./2048.obj").expect("couldn't open file");
    let mut f = BufReader::new(f);

    let base_address = f.read_u16::<BigEndian>().expect("error");

    let mut address = base_address as usize;

    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instruction) => {
                memory[address] = instruction;
                address += 1
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("File read")
                }
                break;
            }
        }
    }

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

        println!("Instruction:{:?} ", instruction_kind);

        match instruction_kind {
            Ok(Instructions::ADD) => add(instruction, &mut registers),
            Ok(Instructions::AND) => and(instruction, &mut registers),
            Ok(Instructions::NOT) => not(instruction, &mut registers),
            Ok(Instructions::BR) => branch(instruction, &mut registers),
            Ok(Instructions::JMP) => jump(instruction, &mut registers),
            Ok(Instructions::JSR) => jump_register(instruction, &mut registers),
            Ok(Instructions::LD) => load(instruction, &mut registers, &memory),
            Ok(Instructions::LDI) => load_indirect(instruction, &mut registers, &memory),
            Ok(Instructions::LDR) => load_register(instruction, &mut registers, &memory),
            Ok(Instructions::LEA) => load_effective_address(instruction, &mut registers, &memory),
            Ok(Instructions::ST) => store(instruction, &registers, &mut memory),
            Ok(Instructions::STI) => store_indirect(instruction, &registers, &mut memory),
            Ok(Instructions::STR) => store_register(instruction, &registers, &mut memory),
            Ok(Instructions::TRAP) => trap(instruction, &mut registers, &memory),
            _ => {
                println!("bad opcode");
                std::process::exit(1);
            }
        }
        print_registers(&registers);
        registers[Registers::RProgramCounter as usize] += 1;
    }
}

fn trap(instruction: u16, registers: &mut [u16; 10], memory: &Memory) {
    registers[Registers::R7 as usize] = registers[Registers::RProgramCounter as usize];

    match TrapCodes::try_from(instruction & 0xFF) {
        Ok(TrapCodes::TrapGetC) => {
            let mut stdin_handle = stdin().lock();
            let mut byte = [0_u8];
            stdin_handle.read_exact(&mut byte).unwrap();

            registers[Registers::R0 as usize] = byte[0] as u16;
            update_flags(Registers::R0 as u16, registers)
        }
        Ok(TrapCodes::TrapOut) => {
            let char = registers[Registers::R0 as usize];
            print!("{}", char);
        }
        Ok(TrapCodes::TrapPuts) => {
            let mut address = registers[Registers::R0 as usize];
            loop {
                let char = memory[address as usize];
                if char == 0 {
                    break;
                }
                print!("{}", char);
                address + 1;
            }
        }
        Ok(TrapCodes::TrapIn) => {}
        Ok(TrapCodes::TrapPutsP) => {}
        Ok(TrapCodes::TrapHalt) => {}
        Err(_) => {
            println!("bad trap code");
            std::process::exit(1);
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

fn and(instruction: u16, registers: &mut [u16; 10]) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extended(instruction & 0x1F, 5);
        registers[r0 as usize] = registers[r1 as usize] & imm5;
    } else {
        let r2 = instruction & 0x7;
        registers[r0 as usize] = registers[r1 as usize] & registers[r2 as usize];
    }
    update_flags(r0, registers);
}

fn not(instruction: u16, registers: &mut [u16; 10]) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;

    registers[r0 as usize] = !registers[r1 as usize];
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

fn load(instruction: u16, registers: &mut [u16; 10], memory: &Memory) {
    let target_register = (instruction >> 9) & 0x7;
    let offset = sign_extended(instruction & 0x1FF, 9);

    let address = offset + registers[Registers::RProgramCounter as usize];

    registers[target_register as usize] = memory[address as usize];

    update_flags(target_register, registers)
}

fn load_indirect(instruction: u16, registers: &mut [u16; 10], memory: &Memory) {
    let target_register = (instruction >> 9) & 0x7;
    let offset = sign_extended(instruction & 0x1FF, 9);

    let pc_offset = offset + registers[Registers::RProgramCounter as usize];

    let address = memory[pc_offset as usize];

    registers[target_register as usize] = memory[address as usize];

    update_flags(target_register, registers)
}

fn load_register(instruction: u16, registers: &mut [u16; 10], memory: &Memory) {
    let target_register: u16 = (instruction >> 9) & 0x7;
    let base_register: u16 = (instruction >> 6) & 0x7;
    let offset = sign_extended(instruction & 0x3F, 6);

    registers[target_register as usize] =
        memory[(registers[base_register as usize] as usize) + (offset as usize)];

    update_flags(target_register, registers)
}

fn load_effective_address(instruction: u16, registers: &mut [u16; 10], memory: &Memory) {
    let target_register = (instruction >> 9) & 0x7;
    let pc_offset = sign_extended(instruction & 0x1FF, 9);

    registers[target_register as usize] =
        registers[Registers::RProgramCounter as usize] + pc_offset;

    update_flags(target_register, registers)
}

fn store(instruction: u16, registers: &[u16; 10], memory: &mut Memory) {
    let target_register = (instruction >> 9) & 0x7;
    let pc_offset = sign_extended(instruction & 0x1FF, 9);
    memory[registers[Registers::RProgramCounter as usize] as usize + (pc_offset as usize)] =
        registers[target_register as usize]
}

fn store_indirect(instruction: u16, registers: &[u16; 10], memory: &mut Memory) {
    let target_register = (instruction >> 9) & 0x7;
    let pc_offset = sign_extended(instruction & 0x1FF, 9);

    memory[memory[registers[Registers::RProgramCounter as usize] as usize + pc_offset as usize]
        as usize] = registers[target_register as usize]
}

fn store_register(instruction: u16, registers: &[u16; 10], memory: &mut Memory) {
    let r0 = (instruction >> 9) & 0x7;
    let r1 = (instruction >> 6) & 0x7;
    let offset = sign_extended(instruction & 0x3F, 6);
    memory[registers[r1 as usize] as usize + offset as usize] = registers[r0 as usize];
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
    vm()
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

    #[test]
    fn test_load() {
        let instruction = 0b0010_010_0000000100;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];
        let mut memory: [u16; 1 << 16] = [0; 1 << 16];

        registers[Registers::RProgramCounter as usize] = 1 << 8;

        let offset =
            registers[Registers::RProgramCounter as usize] + sign_extended(0b0000000100, 8);

        memory[offset as usize] = 1 << 15;

        load(instruction, &mut registers, &memory);

        if DEBUG_PRINT {
            print_registers(&registers);
        }

        assert_eq!(registers[Registers::R4 as usize], memory[offset as usize]);
        assert_eq!(
            registers[Registers::RConditional as usize],
            ConditionalFlags::Negative as u16
        );
    }

    #[test]
    fn test_load_indirect() {
        let instruction = 0b0010_010_0000000100;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];
        let mut memory: [u16; 1 << 16] = [0; 1 << 16];

        registers[Registers::RProgramCounter as usize] = 1 << 8;

        let pc_offset =
            registers[Registers::RProgramCounter as usize] + sign_extended(0b0000000100, 8);

        memory[pc_offset as usize] = 1 << 15;
        memory[memory[pc_offset as usize] as usize] = 1 << 14;

        load_indirect(instruction, &mut registers, &memory);

        if DEBUG_PRINT {
            print_registers(&registers);
        }

        assert_eq!(registers[Registers::R4 as usize], memory[1 << 15]);
        assert_eq!(
            registers[Registers::RConditional as usize],
            ConditionalFlags::Positive as u16
        );
    }

    #[test]
    fn test_load_register() {
        let instruction = 0b0110_010_000_0000100;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];
        let memory: [u16; 1 << 16] = [0; 1 << 16];

        registers[Registers::R2 as usize] = 1 << 8;

        load_register(instruction, &mut registers, &memory);

        if DEBUG_PRINT {
            print_registers(&registers);
        }

        assert_eq!(registers[Registers::R0 as usize], memory[1 << 8]);
        assert_eq!(
            registers[Registers::RConditional as usize],
            ConditionalFlags::Zero as u16
        );
    }
}
