use std::fs::File;
use std::io::Read;
use stalfos_vm::stalfos;
use stalfos_vm::assembler::assembler;
use stalfos_example_programs::example_programs;
use stalfos_vm::assembler::assembler::assemble;
use stalfos_vm::assembler::assembler::parse_binary;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: stalfos-vm <program>");
        return;
    }

    let path = args[1].clone();
    //compile hello_world to that path

    let hello_world = example_programs::jmp_except_catch();
    let hello_world_asm = assemble(&hello_world);
    assembler::write_to_file(&hello_world_asm, &path);

    //read all bytes from file
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let program = parse_binary(buffer);

    let vm = stalfos::VM::run_new(program);

    println!("{:?}", vm.alloc_table);

}
