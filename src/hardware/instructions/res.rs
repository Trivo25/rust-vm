use std::process;

use crate::hardware::registers::Registers;

/**
Unused, abort.
*/
pub fn res(_instruction: u16, _registers: &mut Registers) {
    process::abort()
}
