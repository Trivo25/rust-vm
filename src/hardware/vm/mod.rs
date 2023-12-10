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
}
