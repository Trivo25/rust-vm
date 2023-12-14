use super::{memory::Memory, registers::Registers};

pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod res;
pub mod rti;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

pub fn execute_instruction(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let opcode = instruction >> 12;
    let instruction_kind = Instructions::try_from(opcode);
    println!("Executing instruction: {:?}", instruction_kind);
    match instruction_kind {
        Ok(Instructions::ADD) => br::br(instruction, registers),
        Ok(Instructions::AND) => and::and(instruction, registers),
        Ok(Instructions::NOT) => not::not(instruction, registers),
        Ok(Instructions::BR) => br::br(instruction, registers),
        Ok(Instructions::JMP) => jmp::jmp(instruction, registers),
        Ok(Instructions::JSR) => jsr::jsr(instruction, registers),
        Ok(Instructions::LD) => ld::ld(instruction, registers, memory),
        Ok(Instructions::LDI) => ldi::ldi(instruction, registers, memory),
        Ok(Instructions::LDR) => ldr::ldr(instruction, registers, memory),
        Ok(Instructions::LEA) => lea::lea(instruction, registers),
        Ok(Instructions::ST) => st::st(instruction, registers, memory),
        Ok(Instructions::STI) => sti::sti(instruction, registers, memory),
        Ok(Instructions::STR) => str::str(instruction, registers, memory),
        Ok(Instructions::TRAP) => trap::trap(instruction, registers, memory),
        Ok(Instructions::RTI) => rti::rti(instruction, registers),
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

#[derive(Debug)]
pub enum Instructions {
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

pub fn sign_extended(x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

pub fn update_flags(r: u16, registers: &mut Registers) {
    if registers.read(r) == 0 {
        registers.update_cond_flag(ConditionalFlags::Zero);
    } else if registers.read(r) >> 15 == 1 {
        registers.update_cond_flag(ConditionalFlags::Negative);
    } else {
        registers.update_cond_flag(ConditionalFlags::Positive);
    }
}

pub enum ConditionalFlags {
    Positive = 1 << 0, // P
    Zero = 1 << 1,     // Z
    Negative = 1 << 2, // N
}
