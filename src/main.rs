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

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let program = parse_binary(buffer);

    stalfos::VM::run_new(program);
}
