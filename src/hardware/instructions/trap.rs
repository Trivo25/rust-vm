use std::io::{self, stdin, Read, Write};

use crate::hardware::{
    memory::{self, Memory},
    registers::Registers,
};

pub fn trap(instruction: u16, registers: &mut Registers, memory: &mut Memory) {
    let trap_code = instruction & 0xFF;

    match trap_code {
        0x20 => trap_getc(registers),
        0x21 => trap_out(registers),
        0x22 => trap_puts(registers, memory),
        0x23 => trap_in(registers),
        0x24 => trap_putsp(registers, memory),
        0x25 => trap_halt(),
        _ => panic!("Unknown trap code: {}", trap_code),
    }
}

fn trap_getc(registers: &mut Registers) {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();

    registers.update(0, buffer[0] as u16);
}

fn trap_out(registers: &mut Registers) {
    let c = registers.read(0);
    print!("{}", c as u8 as char);
}

fn trap_puts(registers: &mut Registers, memory: &mut Memory) {
    let mut index = registers.read(0);
    let mut c = memory.read(index);

    while c != 0x0000 {
        print!("{}", c as u8 as char);
        index += 1;
        c = memory.read(index);
    }
    io::stdout().flush().expect("Error flushing output");
}

fn trap_in(registers: &mut Registers) {
    print!("Enter a  character : ");
    io::stdout().flush().expect("failed to flush");
    let char = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16)
        .unwrap();
    registers.update(0, char);
}

fn trap_putsp(registers: &mut Registers, memory: &mut Memory) {
    // Putsp
    let mut index = registers.read(0);
    let mut c = memory.read(index);
    while c != 0x0000 {
        let c1 = ((c & 0xFF) as u8) as char;
        print!("{}", c1);
        let c2 = ((c >> 8) as u8) as char;
        if c2 != '\0' {
            print!("{}", c2);
        }
        index += 1;
        c = memory.read(index);
    }
    io::stdout().flush().expect("failed to flush");
}

fn trap_halt() {
    println!("HALT detected");
    io::stdout().flush().expect("failed to flush");
    std::process::exit(1);
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::hardware::memory;

    #[test]
    fn test_trap_puts() {
        let mut registers = Registers::initial();
        let mut memory = memory::Memory::empty();

        registers.update(0, 0x3000);

        memory.write(0x3000, 'h' as u16);
        memory.write(0x3001, 'e' as u16);
        memory.write(0x3002, 'l' as u16);
        memory.write(0x3003, 'l' as u16);
        memory.write(0x3004, 'o' as u16);
        memory.write(0x3005, 0x0000);

        trap_puts(&mut registers, &mut memory);

        registers.pretty_print()
    }
}
