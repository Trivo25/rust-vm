use super::instructions::ConditionalFlags;

const PROGRAM_COUNTER_START: u16 = 0x3000;

pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
}

impl Registers {
    pub fn initial() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: PROGRAM_COUNTER_START,
            cond: ConditionalFlags::Zero as u16,
        }
    }

    pub fn update(&mut self, register_index: u16, value: u16) {
        match register_index {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.pc = value,
            9 => self.cond = value,
            _ => panic!("Invalid register entry"),
        }
    }

    pub fn update_program_counter(&mut self, value: u16) {
        self.update(8, value)
    }

    pub fn update_cond_flag(&mut self, value: ConditionalFlags) {
        self.update(9, value as u16)
    }

    pub fn read(&self, register: u16) -> u16 {
        match register {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            _ => panic!("Invalid register entry"),
        }
    }

    pub fn read_cond_flag(&self) -> ConditionalFlags {
        match self.cond {
            1 => ConditionalFlags::Positive,
            2 => ConditionalFlags::Zero,
            4 => ConditionalFlags::Negative,
            _ => panic!("Invalid conditional flag entry"),
        }
    }

    pub fn read_program_counter(&self) -> u16 {
        self.pc
    }

    pub fn increment_program_counter(&mut self) {
        self.pc += 1;
    }

    pub fn pretty_print(&self) {
        println!("Registers:");
        println!("R0: 0x{:04X}", self.r0);
        println!("R1: 0x{:04X}", self.r1);
        println!("R2: 0x{:04X}", self.r2);
        println!("R3: 0x{:04X}", self.r3);
        println!("R4: 0x{:04X}", self.r4);
        println!("R5: 0x{:04X}", self.r5);
        println!("R6: 0x{:04X}", self.r6);
        println!("R7: 0x{:04X}", self.r7);
        println!("PC: 0x{:04X}", self.pc);
        println!("COND: 0x{:04X}", self.cond);
    }
}
