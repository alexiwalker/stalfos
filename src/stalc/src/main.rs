use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use stalfos_vm::asm_parser::asm_parser::parse_string;
use stalfos_vm::assembler::assembler;

/*
* STALC : Stalfos ASM (sta) Compiler
* Copyright (C) 2022 Alexander Walker

* Usage: stacl <inputfile.sta> <outputfile.stf> [-r,--run] [--check] [-d, --debug]
*/
fn main() {

    //get args: first is input, second is output
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
        panic!("Usage: stalfos <input> <output>");
    }

    //take args after 3 to check for flags
    let mut run = false;
    let mut check = false;
    let mut debug = false;
    for i in 3..args.len() {
        if args[i] == "-r" || args[i] == "--run" {
            run = true;
        } else if args[i] == "--check" {
            check = true;
        }
        else if args[i] == "-d" || args[i] == "--debug" {
            debug = true;
        }

    }

    let infile = &args[1];
    let outfile = &args[2];

    //read input file
    let mut input = String::new();
    let mut file = File::open(infile).expect("file not found");
    file.read_to_string(&mut input).expect("something went wrong reading the file");

    let (ns,ops) = parse_string(input);

    let binary = assembler::assemble(ops.borrow(),ns.clone());

    if check {
        let (new_ops,_) =assembler::parse_binary(binary.clone());
        let new_binary = assembler::assemble(new_ops.borrow(),ns.clone());
        if binary != new_binary {
            println!("{:?}", binary);
            println!("{:?}", new_binary);
            panic!("Binary is invalid");
        }
    }


    assembler::write_to_file(binary.borrow(), outfile);

    if run {
        if debug {
            stalfos_vm::stalfos::VM::run_new_debug(ops);
        } else {
            stalfos_vm::stalfos::VM::run_new(ops);
        }
    }
}
