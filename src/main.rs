pub mod hardware;

use hardware::vm::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::create();
    vm.load_program("rogue.obj");
    vm.execute_program();
}
