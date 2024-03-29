extern crate core;

pub mod asm_parser;
pub mod assembler;
pub mod ops;

mod op_calls;
mod stal_dll;

pub mod stalfos {
    use crate::op_calls;
    pub use crate::ops::ops;
    use crate::ops::ops::Operator;
    use crate::stal_dll::stal_dll::{StalDynamicInvocation, StalDynamicLibrary};
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::{BTreeMap, HashMap};

    pub struct VM {
        pub stack: Vec<u32>,
        pub program_counter: usize,
        pub memory: Vec<u32>,
        pub program: Vec<ops::Operator>,

        //<preset pointer, (location, size)>
        pub static_alloc_table: BTreeMap<usize, (usize, u32)>,
        pub dynamic_allocations:Vec<(usize,u32)>,
        // label, address
        pub jmp_table: HashMap<String, usize>,

        // from, to
        pub stack_frame_pointers: Vec<(usize, usize)>,

        // output words, from EMIT functions
        pub output: Vec<u32>,

        //if set after an operation is called, this will break and exit
        pub signal_finished: bool,

        // controlls if certain operations will execute. Certain debug operations will not execute
        pub signal_debug: bool,

        // if the last arithmetic operation overflowed, this will be set
        pub signal_overflow: bool,

        // libs have no main function and cannot be run individually. Can be loaded and called later.
        pub is_lib: bool,

        // 16 bytes is sufficient to perform an operation on 2 64bit numbers.
        // essentially this is 2 64bit registers that can be operated on in chunks for
        //smaller operations
        pub registers: [u8; 16],

        // 128but op registers: used to perform a 128bit operation. vm.registers form one 128 bit number (8*16) and vm._128bit_registers form the other
        pub _128_registers: [u8; 16],
    }

    impl VM {
        pub fn new() -> VM {
            VM {
                stack: vec![],
                memory: vec![],
                jmp_table: HashMap::new(),
                stack_frame_pointers: vec![],
                output: vec![],

                //dict that maps a preset value to a memory address
                static_alloc_table: BTreeMap::new(),
                program: vec![],
                program_counter: 0,
                signal_finished: false,
                signal_debug: false,
                signal_overflow: false,
                is_lib: false,
                registers: [0; 16],
                _128_registers: [0; 16],
                dynamic_allocations: vec![],
            }
        }

        pub fn run_new(program: Vec<ops::Operator>) -> VM {
            let mut vm = VM::new();
            vm.add_ops(program).prepare().run();
            vm
        }

        pub fn run_new_debug(program: Vec<ops::Operator>) -> VM {
            let mut vm = VM::new_debug();
            vm.add_ops(program).prepare().run();
            vm
        }

        pub fn new_debug() -> VM {
            VM {
                stack: vec![],
                memory: vec![],
                jmp_table: HashMap::new(),
                stack_frame_pointers: vec![],
                output: vec![],
                //dict that maps a preset value to a memory address
                static_alloc_table: BTreeMap::new(),
                program: vec![],
                program_counter: 0,
                signal_finished: false,
                signal_debug: true,
                signal_overflow: false,
                is_lib: false,
                registers: [0; 16],
                _128_registers: [0; 16],
                dynamic_allocations: vec![],
            }
        }

        pub fn execute_program(&mut self, program: Vec<ops::Operator>) -> &mut VM {
            self.add_ops(program).prepare().run()
        }

        pub fn run(&mut self) -> &mut VM {
            let mut libs = HashMap::new();
            loop {
                if !op_calls::op_calls::execute_operation(self,libs.borrow_mut()) {
                    self.program_counter += 1;
                }

                if self.signal_debug {
                    println!("{}", self.program_counter);
                }

                if self.signal_finished {
                    break;
                }
            }

            return self;
        }

        pub fn run_with_libs(&mut self, libs: &mut HashMap<String, StalDynamicLibrary>) -> &mut VM {
            loop {
                if !op_calls::op_calls::execute_operation(self, libs) {
                    self.program_counter += 1;
                }

                if self.signal_debug {
                    println!("{}", self.program_counter);
                }

                if self.signal_finished {
                    break;
                }
            }

            return self;
        }

        pub fn run_specific_operation(&mut self, operation_number: usize) -> &mut VM {
            // let mut libL:HashMap<String,StalDynamicLibrary> = ;

            // let libraries = L.borrow_mut();

            let pc_before: usize = self.program_counter;
            self.program_counter = operation_number;
            op_calls::op_calls::execute_operation(self, HashMap::new().borrow_mut());
            self.program_counter = pc_before;
            self
        }

        pub fn run_single_operation(&mut self, op: Operator) -> &mut VM {
            let pc_before: usize = self.program_counter;
            self.program.push(op);
            self.program_counter = self.program.len() - 1;
            op_calls::op_calls::execute_operation(self, HashMap::new().borrow_mut());
            self.program.pop();
            self.program_counter = pc_before;
            self
        }

        pub fn add_op(&mut self, op: ops::Operator) -> &mut VM {
            self.program.push(op);
            return self;
        }

        pub fn add_ops(&mut self, ops: Vec<ops::Operator>) -> &mut VM {
            self.program.extend(ops);
            return self;
        }

        fn process_jump_definitions(&mut self) -> () {
            let program_size = self.program.len();
            // let program = self.program
            for op in self.program.iter() {
                match op {
                    Operator::JMP_DEF(key, pointer) => {
                        self.jmp_table.insert(key.to_string(), *pointer);
                    }
                    Operator::JMP_SCAN => {
                        let current_pc = self.program_counter;
                        // let program_size = self.program.len();
                        for i in current_pc..program_size {
                            let op = self.program[i].borrow();
                            match op {
                                Operator::LABEL(key) => {
                                    let label = key.to_string();

                                    if !self.jmp_table.contains_key(&*label) {
                                        self.jmp_table.insert(label, i);
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    _ => return,
                }
            }
            return;
        }

        /**
         * Sets up jump table, finds main and sets the program counter to it
         * @param preset_value
         * @param location
         * @param size
         */
        pub fn prepare(&mut self) -> &mut VM {
            self.process_jump_definitions();
            self.program_counter = 0;

            if self.jmp_table.contains_key("main") {
                self.program_counter = self.jmp_table["main"];
                self.stack_frame_pointers.push((0, self.program_counter))
            } else {
                if !self.is_lib {
                    panic!("No main function found");
                }
            }

            return self;
        }

        pub(crate) fn syscall(syscall_id: usize, mut args: Vec<u32>) -> bool {
            //if you do not have arguments and the syscall requires an argument, a 1 represents false
            if args.is_empty() {
                args.push(1)
            }
            match syscall_id {
                0 => {
                    panic!("VM called a panic! with code {}", args[0]);
                }
                1 => {
                    println!("{}", args[0]);
                }
                2 => {
                    println!("VM ended with exit code {}", args[0]);
                    return false;
                }
                3 => {
                    let string = VM::get_string_from_u32_vec(args);
                    println!("{}", string);
                }
                _ => {
                    println!("Unknown syscall: {}", syscall_id);
                }
            }

            return true;
        }

        pub fn get_string_from_u32_vec(values: Vec<u32>) -> String {
            //convert each u32 into 4 chars
            let mut string = String::new();
            for value in values {
                let mut chars = [0; 4];
                chars.copy_from_slice(&value.to_be_bytes());
                string.push_str(&String::from_utf8_lossy(&chars));
            }

            //remove trailing null chars
            let mut index = string.len() - 1;
            while string.chars().nth(index).unwrap() == '\0' {
                index -= 1;
            }
            string.truncate(index + 1);

            return string;
        }


        /// Allocates memory for a new variable with a known size and a fixed identifier ptr usize
        ///
        /// # Arguments
        ///
        /// * `ptr`: a usize that represents the identifier of the variable
        /// * `size`: the number of words (4 bytes) to allocate
        ///
        /// returns: usize : the pointer to the allocated memory in the heap. Indexing the heap with this value+0 -> value+size-1 are valid for this allocation
        ///

        pub fn allocate(&mut self, ptr: usize, size: u32) -> usize {
            // fn _alloc(vm: &mut VM, ptr: &mut usize, size: &mut u32) -> usize {
            let table = self.static_alloc_table.borrow_mut();

            //get all keys as a vector
            let keys: Vec<usize> = table.keys().map(|x| *x).collect();
            let l = keys.len();
            for x in 0..l {
                let current_pointer = keys.get(x).unwrap();
                let opt_next_pointer = keys.get(x + 1);
                let has_next = opt_next_pointer.is_some();
                // if the current key is not the final one in the allocations, check if the required size fits between current+len and next
                if x < l && has_next {
                    let next_pointer = keys.get(x + 1).unwrap();
                    let v = self.static_alloc_table[&current_pointer];
                    let (stack_location, s) = v;
                    let (next_stack_location, _) = self.static_alloc_table[&next_pointer];
                    if stack_location + s as usize + (size as usize) < next_stack_location {
                        let allocation = (stack_location, s + size);
                        self.static_alloc_table.insert(ptr, allocation);
                        return stack_location;
                    }
                }
            }

            let end_of_stack = self.memory.len();
            let allocation = (end_of_stack, size);
            self.static_alloc_table.insert(ptr, allocation);
            for _ in 0..size {
                self.memory.push(0);
            }

            return end_of_stack;


        }

        pub fn dyn_allocate(&mut self, size:u32) -> usize {
            let table = self.static_alloc_table.borrow_mut();
            //get all keys as a vector
            let keys: Vec<usize> = table.keys().map(|x| *x).collect();
            let l = keys.len();
            for x in 0..l {
                let current_pointer = keys.get(x).unwrap();
                let opt_next_pointer = keys.get(x + 1);
                let has_next = opt_next_pointer.is_some();
                // if the current key is not the final one in the allocations, check if the required size fits between current+len and next
                if x < l && has_next {
                    let next_pointer = keys.get(x + 1).unwrap();
                    let v = self.static_alloc_table[&current_pointer];
                    let (stack_location, s) = v;
                    let (next_stack_location, _) = self.static_alloc_table[&next_pointer];
                    if stack_location + s as usize + (size as usize) < next_stack_location {
                        let allocation = (stack_location, s + size);
                        self.dynamic_allocations.push(allocation);
                        return stack_location;
                    }
                }
            }

            let end_of_stack = self.memory.len();
            let allocation = (end_of_stack, size);
            self.dynamic_allocations.push(allocation);
            for _ in 0..size {
                self.memory.push(0);
            }

            return end_of_stack;

        }

        pub fn get_next_string(&mut self) -> String {
            let n_args = self.stack.pop().unwrap();
            let mut args = Vec::new();
            for _ in 0..n_args {
                args.push(self.stack.pop().unwrap());
            }
            args.reverse();
            VM::get_string_from_u32_vec(args)
        }

        pub fn call_dynamic_library(
            &mut self,
            libraries: &mut HashMap<String, StalDynamicLibrary>,
            library: String,
            label: String,
        ) {
            if libraries.contains_key(&*library) {
                let lib = libraries.get(&*library).unwrap();
                let mut invocation = StalDynamicInvocation::new(lib.clone());
                let results = invocation.call_func(label, self.stack.clone(), libraries);
                self.stack.extend(results);
            } else {
                panic!("Library {} not loaded", library);
            }
        }
    }
}
