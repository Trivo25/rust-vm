pub mod hardware;

use hardware::vm::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::create();
    vm.load_program("2048.obj");
    vm.execute_program();
}
