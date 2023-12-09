use std::usize;

const MEMORY_MAX: u32 = 1 << 16;

#[repr(u16)]
enum Instructions {
    Branch,
    Add,
    Load,
    Store,
    JumpRegister,
    And,
    LoadRegister,
    StoreRegister,
    Unused,
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    Reserved,
    LoadEffectiveAddress,
    Trap,
}

impl TryFrom<i32> for Instructions {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instructions::Branch),
            1 => Ok(Instructions::Add),
            2 => Ok(Instructions::Load),
            3 => Ok(Instructions::Store),
            4 => Ok(Instructions::JumpRegister),
            5 => Ok(Instructions::And),
            6 => Ok(Instructions::LoadRegister),
            7 => Ok(Instructions::StoreRegister),
            8 => Ok(Instructions::Unused),
            9 => Ok(Instructions::Not),
            10 => Ok(Instructions::LoadIndirect),
            11 => Ok(Instructions::StoreIndirect),
            12 => Ok(Instructions::Jump),
            13 => Ok(Instructions::Reserved),
            14 => Ok(Instructions::LoadEffectiveAddress),
            15 => Ok(Instructions::Trap),
            _ => Err(()),
        }
    }
}

#[repr(u16)]
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

#[repr(u16)]
enum ConditionalFlags {
    Positive = 1 << 0, // P
    Zero = 1 << 1,     // Z
    Negative = 1 << 2, // N
}

pub fn read_image(_arg: &String) -> bool {
    true
}

pub fn vm(argc: i32, argv: Vec<String>) {
    if argc < 2 {
        println!("lc3 [image-file1] ...");
        std::process::exit(2);
    }

    for arg in argv {
        println!("arg: {}", arg);
        if !read_image(&arg) {
            println!("failed to load image: {}", arg);
            std::process::exit(1);
        }
    }

    let memory = [0; MEMORY_MAX as usize];
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

        match Instructions::try_from(op_code) {
            Ok(Instructions::Add) => add(instruction as u16, &mut registers),
            Ok(Instructions::And) => {}
            Ok(Instructions::Not) => {}
            Ok(Instructions::Branch) => {}
            Ok(Instructions::JumpRegister) => {}
            Ok(Instructions::Load) => {}
            Ok(Instructions::LoadIndirect) => {}
            Ok(Instructions::LoadEffectiveAddress) => {}
            Ok(Instructions::LoadRegister) => {}
            Ok(Instructions::Store) => {}
            Ok(Instructions::StoreIndirect) => {}
            Ok(Instructions::StoreRegister) => {}
            Ok(Instructions::Jump) => {}
            Ok(Instructions::Trap) => {}
            Ok(Instructions::Reserved) => {}
            Ok(Instructions::Unused) => {}
            Err(_) => {
                println!("Unrecognized opcode");
                break;
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
    for i in 0..(Registers::RTotalCount as usize - 1) {
        println!("R{}: {}", i, registers[i]);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let instruction = 0b0001_000_001_0_00_010;

        let a = 3;
        let b = 5;

        let mut registers: [u16; 10] = [0; Registers::RTotalCount as usize];

        registers[Registers::R1 as usize] = a;
        registers[Registers::R2 as usize] = b;

        add(instruction, &mut registers);

        print_registers(&registers);

        assert_eq!(registers[Registers::R0 as usize], a + b);
    }
}
