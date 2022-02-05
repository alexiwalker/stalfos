pub mod stal_dll {
    use crate::assembler::assembler::parse_binary;
    use crate::stalfos::ops::Operator;
    use crate::stalfos::VM;
    use std::borrow::BorrowMut;
    use std::collections::{BTreeMap, HashMap};
    use std::fs::File;
    use std::io::Read;

    #[derive(Debug, Clone)]
    pub struct StalDynamicLibrary {
        pub namespace: String,
        pub operations: Vec<Operator>,
        pub jump_table: HashMap<String, usize>,
    }

    #[derive(Debug, Clone)]
    pub struct StalDynamicInvocation {
        pub lib: StalDynamicLibrary,
        pub stack: Vec<u32>,
        pub memory: Vec<u32>,
        pub alloc_table: BTreeMap<usize, (usize, u32)>,
    }

    pub fn load_library(namespace: &str) -> StalDynamicLibrary {
        let path = format!("{}.stalib", namespace);
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let (program, namespace) = parse_binary(buffer);

        let mut jump_table = HashMap::new();
        for (_, op) in program.iter().enumerate() {
            match op {
                Operator::JMP_DEF(string, ptr) => {
                    jump_table.insert(string.clone(), *ptr);
                }
                Operator::LABEL(string) => {
                    if string == "JT_END" {
                        break;
                    }
                }
                _ => {}
            }
        }
        StalDynamicLibrary {
            namespace: namespace.to_string(),
            operations: program,
            jump_table,
        }
    }

    pub fn load_file_as_library(path: &str, as_namespace: &str) -> StalDynamicLibrary {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let (program, _) = parse_binary(buffer);

        let mut jump_table = HashMap::new();
        for (_, op) in program.iter().enumerate() {
            match op {
                Operator::JMP_DEF(string, ptr) => {
                    jump_table.insert(string.clone(), *ptr);
                }
                Operator::LABEL(string) => {
                    if string == "JT_END" {
                        break;
                    }
                }
                _ => {}
            }
        }
        StalDynamicLibrary {
            namespace: as_namespace.to_string(),
            operations: program,
            jump_table,
        }
    }

    impl StalDynamicLibrary {
        pub fn new(namespace: String, operations: Vec<Operator>) -> StalDynamicLibrary {
            let mut jump_table = HashMap::new();
            for (i, op) in operations.iter().enumerate() {
                match op {
                    Operator::LABEL(name) => {
                        jump_table.insert(name.clone(), i);
                    }
                    _ => {}
                }
            }
            StalDynamicLibrary {
                namespace,
                operations,
                jump_table,
            }
        }
    }

    impl StalDynamicInvocation {
        pub fn new(lib: StalDynamicLibrary) -> StalDynamicInvocation {
            StalDynamicInvocation {
                lib,
                stack: Vec::new(),
                memory: Vec::new(),
                alloc_table: BTreeMap::new(),
            }
        }

        fn pack_as_vm(&mut self) -> VM {
            let mut v = Vec::new();
            v.extend_from_slice(self.lib.operations.borrow_mut());
            VM {
                program: v,
                stack: self.stack.clone(),
                program_counter: 0,
                memory: self.memory.clone(),
                alloc_table: self.alloc_table.clone(),
                signal_overflow: false,
                signal_finished: false,
                jmp_table: self.lib.jump_table.clone(),
                stack_frame_pointers: Vec::new(),
                output: Vec::new(),
                signal_debug: false,
                is_lib: true,
                registers: [0; 16],
            }
        }

        pub fn call_func(
            &mut self,
            name: String,
            arg_stack: Vec<u32>,
            libs: &mut HashMap<String, StalDynamicLibrary>,
        ) -> Vec<u32> {
            let mut ret: Vec<u32> = vec![];

            let jump_location = *self.lib.jump_table.get(&name).unwrap();
            self.stack.extend_from_slice(&arg_stack);
            let mut vm = self.pack_as_vm();
            vm.prepare();
            vm.program_counter = jump_location;
            vm.run_with_libs(libs);

            let allocation_size = vm.stack.pop();
            if allocation_size != None {
                //copy last n bytes of stack to ret
                let mut i = 0;
                while i < allocation_size.unwrap() {
                    ret.push(vm.stack.pop().unwrap());
                    i += 1;
                }

                ret.reverse();
                ret.push(allocation_size.unwrap() as u32);
            }

            return ret;
        }
    }
}
