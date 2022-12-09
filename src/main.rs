use std::env::set_current_dir;
use std::fs;
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
    //this is a test to see if i can connect to github
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: stalfos-vm <program>");
        return;
    }

    let path = args[1].clone();

    let executing_file =  fs::canonicalize(path.clone()).unwrap();
    let dir =executing_file.parent();

    let dirset=set_current_dir(dir.unwrap());
    dirset.unwrap();



    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();


    file.read_to_end(&mut buffer).unwrap();
    let (program, _) = parse_binary(buffer);

    stalfos::VM::run_new(program);
}
