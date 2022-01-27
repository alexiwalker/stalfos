use stalfos_vm::stalfos;
use stalfos_example_programs::example_programs;

fn main() {

    let program = example_programs::default_example_program();

    let mut vm = stalfos::VM::new();

    vm.execute_program(program);

}
