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

    fn get_registers(&self) -> &Registers {
        &mut self.registers
    }

    pub fn execute_program(&mut self) {
        while (self.registers.read_program_counter() as usize) < (self.memory.memory_max) {
            let next_instruction = self.read_memory(self.registers.read_program_counter());

            self.registers.increment_program_counter();

            let mut reg = self.get_registers();

            execute_instruction(next_instruction, reg, &self.memory)
        }
    }
}
