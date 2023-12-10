pub mod hardware;

use hardware::vm::VirtualMachine;

fn main() {
    let x = VirtualMachine::create();
}
