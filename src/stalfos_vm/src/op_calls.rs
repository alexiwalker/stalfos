pub mod op_calls {
    use std::borrow::{Borrow, BorrowMut};
    use crate::stalfos::ops::Operator;
    use crate::stalfos::VM;

    pub fn execute_operation(vm: &mut crate::stalfos::VM) -> bool {
        let op = vm.program[vm.program_counter].borrow_mut();
        let mut has_changed_ptr = false;
        let mut overflow = false;
        match op {
            Operator::PUSH(v) => {
                vm.stack.push(*v);
            }
            // LOAD only gets the first word (4bytes) of the allocation
            // this means it is only suitable for small allocations
            // use LOADD for larger allocations
            Operator::LOAD(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (stack_location, _) = v;
                let val = vm.memory[stack_location];
                vm.stack.push(val);
            }
            Operator::LOADD(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (stack_location, size) = v;
                let size = size as usize;
                for i in 0..size {
                    let val = vm.memory[stack_location + i];
                    vm.stack.push(val);
                }

                vm.stack.push(size as u32);
            }
            Operator::CONST_U(identifier, value_to_store) => {
                let size = 1;
                let mut allocated_memory_location = 0;
                {
                    let mut _s = false;
                    let table = vm.alloc_table.borrow_mut();

                    //get all keys as a vector
                    let keys: Vec<usize> = table.keys().map(|x| *x).collect();
                    let l = keys.len();
                    for x in 0..l {

                        // if the current key is not the final one in the allocations, check if the required size fits between current+len and next
                        if x < l {
                            let current_pointer = keys.get(x).unwrap();
                            let next_pointer = keys.get(x + 1).unwrap();
                            let v = vm.alloc_table[&current_pointer];
                            let (stack_location, s) = v;
                            let (next_stack_location, _) = vm.alloc_table[&next_pointer];
                            if stack_location + s as usize + (size as usize) < next_stack_location {
                                let allocation = (stack_location, s + size);
                                let p = *identifier;
                                vm.alloc_table.insert(p, allocation);
                                allocated_memory_location = p;
                                _s = true;
                                break;
                            }
                        }
                    }

                    if !_s {
                        let end_of_stack = vm.memory.len();
                        let allocation = (end_of_stack, size);
                        vm.alloc_table.insert(*identifier, allocation);
                        for _ in 0..size {
                            vm.memory.push(0);
                        }

                        allocated_memory_location = end_of_stack
                    }
                }
                vm.memory[allocated_memory_location] = *value_to_store;
            }
            Operator::CONST_F(ptr, v) => {
                let v = *v as u32;
                vm.alloc_table.insert(*ptr, (vm.memory.len(), v));
                vm.memory.push(v as u32);
            }
            Operator::CONST_S(ptr, string) => {
                //split string into byte chunks
                let mut string_bytes = string.as_bytes().to_vec();

                //pad with null bytes to a multiple of 4
                let mut padding = 0;
                if string_bytes.len() % 4 != 0 {
                    padding = 4 - (string_bytes.len() % 4);
                }
                for _ in 0..padding {
                    string_bytes.push(0);
                }

                //split into 4 byte chunks and convert each chunk to u32
                let mut string_chunks = Vec::new();
                for i in 0..string_bytes.len() {
                    if i % 4 == 0 {
                        let mut chunk = [0; 4];
                        chunk[0] = string_bytes[i];
                        chunk[1] = string_bytes[i + 1];
                        chunk[2] = string_bytes[i + 2];
                        chunk[3] = string_bytes[i + 3];
                        string_chunks.push(u32::from_be_bytes(chunk));
                    }
                }

                let size = string_chunks.len() as u32;
                //store the chunks in the memory
                let mut allocated_memory_location = 0;
                {
                    let mut _s = false;
                    let table = vm.alloc_table.borrow_mut();

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
                            let v = vm.alloc_table[&current_pointer];
                            let (stack_location, s) = v;
                            let (next_stack_location, _) = vm.alloc_table[&next_pointer];
                            if stack_location + s as usize + (size as usize) < next_stack_location {
                                let allocation = (stack_location, s + size);
                                let p = *ptr;
                                vm.alloc_table.insert(p, allocation);
                                allocated_memory_location = p;
                                _s = true;
                                break;
                            }
                        }
                    }

                    if !_s {
                        let end_of_stack = vm.memory.len();
                        let allocation = (end_of_stack, size);
                        vm.alloc_table.insert(*ptr, allocation);
                        for _ in 0..size {
                            vm.memory.push(0);
                        }

                        allocated_memory_location = end_of_stack
                    }
                }

                for i in 0..string_chunks.len() {
                    vm.memory[allocated_memory_location + i] = string_chunks[i];
                }
            }
            Operator::CONST_I(ptr, v) => {
                let v = *v as u32;
                vm.alloc_table.insert(*ptr, (vm.memory.len(), v));
                vm.memory.push(v);
            }
            Operator::CONST_B(ptr, v) => {
                let val = if *v { 1 } else { 0 };
                vm.alloc_table.insert(*ptr, (vm.memory.len(), val));
                vm.memory.push(val);
            }
            Operator::LOAD_CONST(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (stack_location, _) = v;
                let val = vm.memory[stack_location];
                vm.stack.push(val);
            }
            Operator::GETLEN(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (_, s) = v;
                vm.stack.push(s);
            }

            Operator::POP => {
                vm.stack.pop();
            }
            Operator::ALLOC(ptr, size) => {
                //check for allocation gaps in the stack
                let table = vm.alloc_table.borrow_mut();

                //get all keys as a vector
                let keys: Vec<usize> = table.keys().map(|x| *x).collect();
                let l = keys.len();
                for x in 0..l {

                    // if the current key is not the final one in the allocations, check if the required size fits between current+len and next
                    if x < l {
                        let sptr = keys.get(x).unwrap();
                        let next = keys.get(x + 1).unwrap();
                        let v = vm.alloc_table[&sptr];
                        let (stack_location, s) = v;
                        let (next_stack_location, _) = vm.alloc_table[&next];
                        if stack_location + s as usize + (*size as usize) < next_stack_location {
                            let allocation = (stack_location, s + *size);
                            let p = *ptr;
                            vm.alloc_table.insert(p, allocation);
                        }
                    }
                }


                let p = vm.memory.len();
                let allocation = (p, *size);
                vm.alloc_table.insert(*ptr, allocation);
                for _ in 0..*size {
                    vm.memory.push(0);
                }
            }
            Operator::POPS(ptr) => {
                //pop and store
                let v = vm.stack.pop().unwrap();
                vm.alloc_table.insert(*ptr, (vm.memory.len(), v));
                vm.memory.push(v);
            }
            Operator::ADDf => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a + b));
            }
            Operator::SUBf => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a - b));
            }
            Operator::MULf => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a * b));
            }
            Operator::DIVf => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a / b));
            }
            Operator::MODf => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a % b));
            }
            Operator::ADDi => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                let (v, o) = i32::overflowing_add(a, b);
                vm.stack.push(i_to_u(v));
                overflow = o;
            }
            Operator::SUBi => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                let (v, o) = i32::overflowing_sub(a, b);
                vm.stack.push(i_to_u(v));
                overflow = o;
            }
            Operator::MULi => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                let (v, o) = i32::overflowing_mul(a, b);
                vm.stack.push(i_to_u(v));
                overflow = o;
            }
            Operator::DIVi => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                let (v, o) = i32::overflowing_div(a, b);
                vm.stack.push(i_to_u(v));
                overflow = o;
            }
            Operator::MODi => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                vm.stack.push(i_to_u(a % b));
            }
            Operator::ADDfi => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a + b as f32));
            }
            Operator::SUBfi => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a - b as f32));
            }
            Operator::MULfi => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a * b as f32));
            }
            Operator::DIVfi => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a / b as f32));
            }
            Operator::MODfi => {
                let a = u_to_f(vm.stack.pop().unwrap());
                let b = u_to_i(vm.stack.pop().unwrap());
                vm.stack.push(f_to_u(a % b as f32));
            }
            Operator::ADDif => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(i_to_u(a + b as i32));
            }
            Operator::SUBif => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(i_to_u(a - b as i32));
            }
            Operator::MULif => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(i_to_u(a * b as i32));
            }
            Operator::DIVif => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(i_to_u(a / b as i32));
            }
            Operator::MODif => {
                let a = u_to_i(vm.stack.pop().unwrap());
                let b = u_to_f(vm.stack.pop().unwrap());
                vm.stack.push(i_to_u(a % b as i32));
            }
            Operator::NEG => {
                let a = vm.stack.pop().unwrap();
                vm.stack.push(!a);
            }
            Operator::AND => {
                let a = vm.stack.pop().unwrap();
                let b = vm.stack.pop().unwrap();
                vm.stack.push(a & b);
            }
            Operator::XOR => {
                let a = vm.stack.pop().unwrap();
                let b = vm.stack.pop().unwrap();
                vm.stack.push(a ^ b);
            }
            Operator::NAND => {
                let a = vm.stack.pop().unwrap();
                let b = vm.stack.pop().unwrap();
                vm.stack.push(!(a & b));
            }
            Operator::CNT => {
                let a = vm.stack.pop().unwrap();
                let mut cnt = 0;
                for i in 0..32 {
                    if a & (1 << i) != 0 {
                        cnt += 1;
                    }
                }
                vm.stack.push(cnt);
            }
            Operator::CMP => {
                let a = vm.stack.pop().unwrap();
                let b = vm.stack.pop().unwrap();
                vm.stack.push(a - b);
            }
            Operator::JMPe(location) => {
                let last_op = vm.stack.pop().unwrap();
                if last_op == 0 {
                    // true is 0 because it is a compare by subtraction: if equal, result is 0
                    let ptr = vm.jmp_table.get(location).unwrap();
                    let before = vm.program_counter;
                    vm.program_counter = *ptr;

                    has_changed_ptr = true;
                    vm.stack_frame_pointers.push((before, vm.program_counter));
                }
            }
            Operator::JMPne(location) => {
                let last_op = vm.stack.pop().unwrap();
                if last_op != 0 {
                    // false is non-0 because it is a compare by subtraction: if equal, result is 0, else false
                    let ptr = vm.jmp_table.get(location).unwrap();
                    let before = vm.program_counter;

                    vm.program_counter = *ptr;

                    has_changed_ptr = true;
                    vm.stack_frame_pointers.push((before, vm.program_counter));
                }
            }
            Operator::SYSCALL(syscall_id, n_args) => {
                let mut args = Vec::new();
                for _ in 0..*n_args {
                    args.push(vm.stack.pop().unwrap());
                }
                args.reverse();
                let program_continue = VM::syscall(*syscall_id, args);

                vm.signal_finished = !program_continue;
            }
            Operator::EXCEPT_THROW => {

                //decrease program counter and inspect its operation until a CATCH is found, deallocating each allocation made in the meantime
                //when it reaches the value of the most recent jump it will jump back to the previous value of the previous jump
                let (mut before, mut after) = vm.stack_frame_pointers.pop().unwrap();

                loop {
                    vm.program_counter -= 1;
                    let op = vm.program[vm.program_counter].borrow();

                    //match op to find catch
                    match op {
                        Operator::EXCEPT_CATCH(catch_location) => {
                            let before = vm.program_counter;
                            let after = vm.jmp_table.get(catch_location).unwrap();
                            vm.stack_frame_pointers.push((before, *after));
                            vm.program_counter = *after;

                            if vm.signal_debug {
                                println!("CATCH FOUND: {},{}", before, catch_location);
                            }
                            //jump to catch
                            break;
                        }

                        _ => {
                            // not a catch, noop. in future will implement deallocations are required
                        }
                    }

                    if vm.program_counter == after {
                        vm.program_counter = before;
                        let temp = vm.stack_frame_pointers.pop().unwrap();
                        before = temp.0;
                        after = temp.1;
                    }
                }
            }
            Operator::EXCEPT_CATCH(_handler) => {
                //noop, catches are handled during a throw unwrap
            }
            Operator::LABEL(str) => {
                // if vm.signal_debug {
                    println!("found label {} at position {}", str, vm.program_counter);
                // }
            }

            Operator::DEALLOC(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (stack_location, size) = v;
                //zero out the memory at the location
                for i in stack_location..(stack_location + (size as usize)) {
                    vm.memory[i] = 0;
                }

                vm.alloc_table.remove(&ptr);
            }
            Operator::JMP_DEF(_, _) => {
                panic!("JMP_DEF found after other instructions");
                //do nothing here, these functions are runtime but the jump definitions are handled externally
                // and should never be handed to this function
                //jump defs require extra handling
            }
            Operator::JMP(location) => {
                let ptr = vm.jmp_table.get(location).unwrap();
                let before = vm.program_counter;

                vm.program_counter = *ptr;

                has_changed_ptr = true;
                vm.stack_frame_pointers.push((before, vm.program_counter));
            }
            Operator::RET => {
                vm.stack_frame_pointers.pop();
                if vm.stack_frame_pointers.len() == 0 {
                    vm.signal_finished = true;
                    vm.program_counter = 0;
                    return true;
                }

                has_changed_ptr = true;

                let (before, _) = vm.stack_frame_pointers.last().unwrap();
                //go to before
                vm.program_counter = *before
            }
            Operator::JMPs(_true, _false) => {
                let last_op = vm.stack.pop().unwrap();

                let ptr = if last_op == 0 {
                    vm.jmp_table.get(_true).unwrap()
                } else {
                    vm.jmp_table.get(_false).unwrap()
                };
                let before = vm.program_counter;
                vm.program_counter = *ptr;
                has_changed_ptr = true;
                vm.stack_frame_pointers.push((before, vm.program_counter));
            }
            Operator::GETBYTELEN(ptr) => {
                let v = vm.alloc_table.get(&ptr).unwrap();
                let (stack_location, size) = v;
                let mut unbuffered_count = (*size) * 4;
                let ptr = (*stack_location) + (*size as usize);
                let word = vm.memory[ptr];
                let bytes = u_to_bytes(word);
                for byte in bytes {
                    if byte == 0 {
                        break;
                    }
                    unbuffered_count += 1;
                }

                vm.stack.push(unbuffered_count);
            }
            Operator::GETBYTE(ptr, offset) => {
                let v = vm.alloc_table.get(&ptr).unwrap();
                let (stack_location, size) = v;
                let mut buffer: Vec<u8> = vec![];

                for i in 0..*size {
                    let word = vm.memory[(*stack_location) + i as usize];
                    let bytes = u_to_bytes(word);
                    buffer.extend(bytes);
                }

                if offset >= &mut buffer.len() {
                    panic!("offset out of bounds");
                }
                let v = buffer.get(*offset as usize).unwrap();

                vm.stack.push(*v as u32);
            }
            Operator::GETWORD(ptr, offset) => {
                let v = vm.alloc_table.get(&ptr).unwrap();
                let (stack_location, _size) = v;
                let loc = (*stack_location) + (*offset);
                let word = vm.memory[loc];
                vm.stack.push(word);
            }
            Operator::SETBYTE(ptr, offset, value) => {
                let v = vm.alloc_table.get(&ptr).unwrap();
                let (stack_location, _size) = v;
                let chunk = (*offset) / 4;
                let offset = (*offset) % 4;
                let loc = (*stack_location) + (chunk);
                let word = vm.memory[loc];
                let mut bytes = u_to_bytes(word);
                bytes[offset] = *value as u8;
                let new_word = bytes_to_u(bytes);
                vm.memory[loc] = new_word;
            }
            Operator::SETWORD(ptr, offset, value) => {
                let v = vm.alloc_table.get(&ptr).unwrap();
                let (stack_location, _size) = v;
                let loc = (*stack_location) + (*offset);
                vm.memory[loc] = *value;
            }
            Operator::JMP_SCAN => {
                //noop, this is run during prepare()
            }
            Operator::ROR => {
                let v = vm.stack.pop().unwrap();
                let v = v.rotate_right(1);
                vm.stack.push(v);
            }
            Operator::ROL => {
                let v = vm.stack.pop().unwrap();
                let v = v.rotate_left(1);
                vm.stack.push(v);
            }
            Operator::LSR => {
                let v = vm.stack.pop().unwrap();
                let v = v >> 1;
                vm.stack.push(v);
            }
            Operator::ASR => {
                let v = vm.stack.pop().unwrap();
                let v = ((v as i32) >> 1) as u32;
                vm.stack.push(v);
            }
            Operator::LSL => {
                let v = vm.stack.pop().unwrap();
                let v = v << 1;
                vm.stack.push(v);
            }
            Operator::ASL => {
                let v = vm.stack.pop().unwrap();
                let v = ((v as i32) << 1) as u32;
                vm.stack.push(v);
            }
            Operator::ADDu => {
                let v1 = vm.stack.pop().unwrap();
                let v2 = vm.stack.pop().unwrap();
                let (v, o) = u32::overflowing_add(v1, v2);
                vm.stack.push(v);
                overflow = o;
            }
            Operator::SUBu => {
                let v1 = vm.stack.pop().unwrap();
                let v2 = vm.stack.pop().unwrap();
                let (v, o) = u32::overflowing_sub(v1, v2);
                vm.stack.push(v);
                overflow = o;
            }
            Operator::MULu => {
                let v1 = vm.stack.pop().unwrap();
                let v2 = vm.stack.pop().unwrap();
                let (v, o) = u32::overflowing_mul(v1, v2);
                vm.stack.push(v);
                overflow = o;
            }
            Operator::DIVu => {
                let v1 = vm.stack.pop().unwrap();
                let v2 = vm.stack.pop().unwrap();
                let (v, o) = u32::overflowing_div(v1, v2);
                vm.stack.push(v);
                overflow = o;
            }
            Operator::MODu => {
                let v1 = vm.stack.pop().unwrap();
                let v2 = vm.stack.pop().unwrap();
                vm.stack.push(v1 % v2);
            }
            Operator::JMPo(location) => {
                if vm.signal_overflow {
                    let ptr = vm.jmp_table.get(location).unwrap();
                    let before = vm.program_counter;

                    vm.program_counter = *ptr;

                    has_changed_ptr = true;
                    vm.stack_frame_pointers.push((before, vm.program_counter));
                }
            }
            Operator::SYSCALLD(syscall_id) => {
                let n_args = vm.stack.pop().unwrap();
                let mut args = Vec::new();
                for _ in 0..n_args {
                    args.push(vm.stack.pop().unwrap());
                }
                args.reverse();
                let program_continue = VM::syscall(*syscall_id, args);

                vm.signal_finished = !program_continue;
            }
            Operator::EMIT => {
                let v = vm.stack.pop().unwrap();
                vm.output.push(v);
            }
            Operator::EMITS(_v) => {
                // get the string from the memory as u32 chunks, convert each to 4 chars, convert chars to string
                //will do in the future
            }
            Operator::EMITW(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (stack_location, _) = v;
                let val = vm.memory[stack_location];
                vm.output.push(val);
            }
            Operator::EMITD(ptr) => {
                let v = vm.alloc_table[&ptr];
                let (stack_location, size) = v;
                let size = size as usize;
                for i in 0..size {
                    let val = vm.memory[stack_location + i];
                    vm.output.push(val);
                }

                vm.output.push(size as u32);
            }
            Operator::DUP => {
                let v = vm.stack.last().unwrap().clone();
                vm.stack.push(v);
            }
            Operator::DUPO(offset) => {
                let v = vm.stack[vm.stack.len() - *offset].clone();
                vm.stack.push(v);
            },
            Operator::SWAP => {
                let v1 = vm.stack.pop().unwrap();
                let v2 = vm.stack.pop().unwrap();
                vm.stack.push(v1);
                vm.stack.push(v2);
            }
        }

        vm.signal_overflow = overflow;

        has_changed_ptr
    }


    fn f_to_bytes(f: f32) -> [u8; 4] {
        f.to_be_bytes()
    }

    fn u_to_bytes(u: u32) -> [u8; 4] {
        u.to_be_bytes()
    }

    fn i_to_bytes(i: i32) -> [u8; 4] {
        i.to_be_bytes()
    }

    fn bytes_to_u(bytes: [u8; 4]) -> u32 {
        u32::from_be_bytes(bytes)
    }

    fn bytes_to_f(bytes: [u8; 4]) -> f32 {
        f32::from_be_bytes(bytes)
    }

    fn bytes_to_i(bytes: [u8; 4]) -> i32 {
        return i32::from_be_bytes(bytes);
    }

    fn u_to_f(u: u32) -> f32 {
        let bytes = u_to_bytes(u);
        bytes_to_f(bytes)
    }

    fn f_to_u(f: f32) -> u32 {
        let bytes = f_to_bytes(f);
        bytes_to_u(bytes)
    }

    fn i_to_u(i: i32) -> u32 {
        let bytes = i_to_bytes(i);
        bytes_to_u(bytes)
    }

    fn u_to_i(u: u32) -> i32 {
        let bytes = u_to_bytes(u);
        bytes_to_i(bytes)
    }
}