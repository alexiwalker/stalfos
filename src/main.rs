use stalfos_vm::stalfos;
use stalfos_example_programs::example_programs;

fn main() {

    let program = example_programs::hello_world();

    let mut vm = stalfos::VM::new();

    vm.execute_program(program);

}
