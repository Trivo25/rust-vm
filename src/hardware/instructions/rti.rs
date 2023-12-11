use std::process;

use crate::hardware::registers::Registers;

/**
Unused, abort.
*/
pub fn rti(_instruction: u16, _registers: &mut Registers) {
    process::abort()
}
