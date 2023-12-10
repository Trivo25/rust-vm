const MEMORY_MAX: usize = u16::MAX as usize;

pub struct Memory {
    memory: [u16; MEMORY_MAX],
    memory_max: usize,
}

impl Memory {
    pub fn empty() -> Memory {
        Memory {
            memory: [0; MEMORY_MAX],
            memory_max: MEMORY_MAX,
        }
    }

    pub fn read(&self, index: u16) -> u16 {
        self.memory[index as usize]
    }

    pub fn write(&mut self, index: u16, value: u16) {
        self.memory[index as usize] = value;
    }
}
