pub mod hardware;

use hardware::vm::VirtualMachine;

fn main() {
    let vm = VirtualMachine::create();
}
