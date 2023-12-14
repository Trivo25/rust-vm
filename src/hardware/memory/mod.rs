use std::io::Read;

const MEMORY_MAX: usize = u16::MAX as usize;

pub struct Memory {
    memory: [u16; MEMORY_MAX],
    pub memory_max: usize,
}
enum MemoryMappedRegister {
    MR_KBSR = 0xFE00, /* keyboard status */
    MR_KBDR = 0xFE02, /* keyboard data */
}

impl Memory {
    pub fn empty() -> Memory {
        Memory {
            memory: [0; MEMORY_MAX],
            memory_max: MEMORY_MAX,
        }
    }

    fn handle_keyboard(&mut self) {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write(MemoryMappedRegister::MR_KBSR as u16, 1 << 15);
            self.write(MemoryMappedRegister::MR_KBDR as u16, buffer[0] as u16);
        } else {
            self.write(MemoryMappedRegister::MR_KBSR as u16, 0)
        }
    }

    pub fn read(&mut self, index: u16) -> u16 {
        if index == MemoryMappedRegister::MR_KBSR as u16 {
            self.handle_keyboard();
        }
        self.memory[index as usize]
    }

    pub fn write(&mut self, index: u16, value: u16) {
        self.memory[index as usize] = value;
    }
}
