use stalfos_vm::assembler::assembler::parse_binary;
use stalfos_vm::stalfos;
use std::fs::File;
use std::io::Read;

/*
* Stalfos : Stalfos Virtual Machine
* Copyright (C) 2022 Alexander Walker

* Usage: stalfos <inputfile.sta>
*/
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
    let (program, _) = parse_binary(buffer);

    stalfos::VM::run_new(program);
}
