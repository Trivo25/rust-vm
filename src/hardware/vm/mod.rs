use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use super::instructions::execute_instruction;
use super::memory::Memory;
use super::registers::Registers;

pub struct VirtualMachine {
    pub memory: Memory,
    pub registers: Registers,
}

impl VirtualMachine {
    pub fn create() -> VirtualMachine {
        VirtualMachine {
            memory: Memory::empty(),
            registers: Registers::initial(),
        }
    }

    pub fn read_memory(&mut self, address: u16) -> u16 {
        self.memory.read(address)
    }

    pub fn read_register(&self, register_index: u16) -> u16 {
        self.registers.read(register_index)
    }

    pub fn execute_program(&mut self) {
        while (self.registers.read_program_counter() as usize) < (self.memory.memory_max) {
            let next_instruction = self.read_memory(self.registers.read_program_counter());

            self.registers.increment_program_counter();

            let registers = &mut self.registers;
            let memory = &mut self.memory;

            execute_instruction(next_instruction, registers, memory);
        }
    }

    pub fn load_program(&mut self, path: &str) {
        let f = File::open(path).unwrap();
        let mut file_buffer = BufReader::new(f);

        let base_address = file_buffer
            .read_u16::<BigEndian>()
            .expect("Error reading file");

        let mut address = base_address;

        loop {
            match file_buffer.read_u16::<BigEndian>() {
                Ok(instruction) => {
                    self.memory.write(address, instruction);
                    address += 1;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        println!("Finished loading program")
                    } else {
                        println!("failed: {}", e);
                    }
                    break;
                }
            }
        }
    }
}
