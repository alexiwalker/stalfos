use stalfos_vm::stalfos;
use stalfos_vm::assembler::assembler;
use stalfos_example_programs::example_programs;

fn main() {
    let program = example_programs::string_manipulation();

    let original_binary = assembler::program_to_binary_format(&program);

    let original_binary_length = original_binary.len();

    let program = assembler::stream_operations_from_binary(original_binary);

    let replicated_binary = assembler::program_to_binary_format(&program);

    println!("original: {}, replicated:{}", original_binary_length, replicated_binary.len());



    let mut vm = stalfos::VM::new();
//     //
    vm.execute_program(program);


    let mut vm2 = stalfos::VM::new();
//     //
    vm2.execute_program(example_programs::string_manipulation());
}
