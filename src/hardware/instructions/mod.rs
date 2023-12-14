use self::conditional_flags::ConditionalFlags;

use super::registers::Registers;

pub mod add;
pub mod and;
pub mod br;
pub mod conditional_flags;
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
