pub mod assembler {
    use crate::ops::ops::Operator;
    use std::fs::File;
    use std::io::Write;
    use std::mem;

    pub fn assemble(program: &Vec<Operator>) -> Vec<u8> {
        let mut val: Vec<u8> = Vec::new();

        //deadface bytes are unique to the stalfos vm, marks the binary as for this project
        let magic_bytes: [u8; 4] = 0xDEADFACE_u32.to_be_bytes();
        val.extend_from_slice(&magic_bytes);
        for operation in program {
            match operation {
                /*opcode : 1*/
                Operator::PUSH(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x01];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 2*/
                Operator::LOAD(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x02];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 3*/
                Operator::LOADD(v_) => {
                    let mut op_bytes: Vec<u8> = vec![0x03];
                    op_bytes.extend_from_slice(&v_.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 4*/
                Operator::CONST_U(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x04];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 5*/
                Operator::CONST_F(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x05];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 6*/
                Operator::CONST_I(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x06];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 7*/
                Operator::CONST_B(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x07];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    let _v2 = if *v2 { 0xff } else { 0x00 };
                    op_bytes.push(_v2);
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 8*/
                Operator::CONST_S(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x08];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    let sbytes = &*str_op_value_bytes(&v2);
                    op_bytes.extend_from_slice(sbytes);
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode : 9*/
                Operator::LOAD_CONST(v) => {
                    //0x09
                    let mut op_bytes: Vec<u8> = vec![0x09];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :10*/
                Operator::POP => {
                    //0x0a
                    val.push(0x0a);
                }
                /*opcode :11*/
                Operator::ALLOC(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x0B];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :12*/
                Operator::DEALLOC(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x0C];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :13*/
                Operator::POPS(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x0D];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :14*/
                Operator::GETLEN(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x0E];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :15*/
                Operator::JMP(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x0F];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v));
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :16*/
                Operator::JMPo(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x10];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v));
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :17*/
                Operator::JMPe(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x11];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v));
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :18*/
                Operator::JMPne(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x12];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v));

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :19*/
                Operator::JMPs(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x13];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v1));
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v2));
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :20*/
                Operator::JMP_DEF(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x14];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v1));
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :21*/
                Operator::LABEL(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x15];
                    let sbytes = &*str_op_value_bytes(&v);
                    op_bytes.extend_from_slice(sbytes);

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :22*/
                Operator::SYSCALL(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x16];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :23*/
                Operator::SYSCALLD(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x17];
                    op_bytes.extend_from_slice(&v.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :24*/
                Operator::EXCEPT_THROW => {
                    val.push(0x18);
                }
                /*opcode :25*/
                Operator::EXCEPT_CATCH(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x19];
                    op_bytes.extend_from_slice(&*str_op_value_bytes(&v));

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :26*/
                Operator::RET => {
                    val.push(0x1A);
                }
                /*opcode :27*/
                Operator::EMIT => {
                    val.push(0x1b);
                }
                /*opcode :28*/
                Operator::EMITS(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x1C];
                    op_bytes.extend_from_slice(&v.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :29*/
                Operator::EMITW(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x1D];
                    op_bytes.extend_from_slice(&v.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :30*/
                Operator::EMITD(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x1E];
                    op_bytes.extend_from_slice(&v.to_be_bytes())
                }
                /*opcode :31*/
                Operator::GETBYTELEN(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x1F];
                    op_bytes.extend_from_slice(&v.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :32*/
                Operator::GETBYTE(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x20];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :33*/
                Operator::GETWORD(v1, v2) => {
                    let mut op_bytes: Vec<u8> = vec![0x21];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :34*/
                Operator::SETBYTE(v1, v2, v3) => {
                    let mut op_bytes: Vec<u8> = vec![0x22];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    op_bytes.push(*v3);

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :35*/
                Operator::SETWORD(v1, v2, v3) => {
                    let mut op_bytes: Vec<u8> = vec![0x23];
                    op_bytes.extend_from_slice(&v1.to_be_bytes());
                    op_bytes.extend_from_slice(&v2.to_be_bytes());
                    op_bytes.extend_from_slice(&v3.to_be_bytes());

                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :36*/
                Operator::DUPO(v) => {
                    let mut op_bytes: Vec<u8> = vec![0x24];
                    op_bytes.extend_from_slice(&v.to_be_bytes());
                    val.extend_from_slice(&op_bytes);
                }
                /*opcode :37*/ Operator::DUP => val.push(0x25),
                /*opcode :38*/ Operator::SWAP => val.push(0x26),
                /*opcode :39*/ Operator::ADDu => val.push(0x27),
                /*opcode :40*/ Operator::ADDi => val.push(0x28),
                /*opcode :41*/ Operator::ADDfi => val.push(0x29),
                /*opcode :42*/ Operator::ADDif => val.push(0x2A),
                /*opcode :43*/ Operator::ADDf => val.push(0x2B),
                /*opcode :44*/ Operator::SUBu => val.push(0x2C),
                /*opcode :45*/ Operator::SUBi => val.push(0x2D),
                /*opcode :46*/ Operator::SUBfi => val.push(0x2E),
                /*opcode :47*/ Operator::SUBif => val.push(0x2F),
                /*opcode :48*/ Operator::SUBf => val.push(0x30),
                /*opcode :49*/ Operator::MULu => val.push(0x31),
                /*opcode :50*/ Operator::MULi => val.push(0x32),
                /*opcode :51*/ Operator::MULfi => val.push(0x33),
                /*opcode :52*/ Operator::MULif => val.push(0x34),
                /*opcode :53*/ Operator::MULf => val.push(0x35),
                /*opcode :54*/ Operator::DIVu => val.push(0x36),
                /*opcode :55*/ Operator::DIVi => val.push(0x37),
                /*opcode :56*/ Operator::DIVfi => val.push(0x38),
                /*opcode :57*/ Operator::DIVif => val.push(0x39),
                /*opcode :58*/ Operator::DIVf => val.push(0x3A),
                /*opcode :59*/ Operator::MODu => val.push(0x3B),
                /*opcode :60*/ Operator::MODi => val.push(0x3C),
                /*opcode :61*/ Operator::MODfi => val.push(0x3D),
                /*opcode :62*/ Operator::MODif => val.push(0x3E),
                /*opcode :63*/ Operator::MODf => val.push(0x3F),
                /*opcode :64*/ Operator::ROR => val.push(0x40),
                /*opcode :65*/ Operator::ROL => val.push(0x41),
                /*opcode :66*/ Operator::LSR => val.push(0x42),
                /*opcode :67*/ Operator::ASR => val.push(0x43),
                /*opcode :68*/ Operator::LSL => val.push(0x44),
                /*opcode :69*/ Operator::ASL => val.push(0x45),
                /*opcode :70*/ Operator::NEG => val.push(0x46),
                /*opcode :71*/ Operator::AND => val.push(0x47),
                /*opcode :72*/ Operator::XOR => val.push(0x48),
                /*opcode :73*/ Operator::NAND => val.push(0x49),
                /*opcode :74*/ Operator::CNT => val.push(0x4A),
                /*opcode :75*/ Operator::CMP => val.push(0x4B),
                /*opcode :76*/ Operator::JMP_SCAN => val.push(0x4C),
                /*opcode :77*/ Operator::OR => {
                    val.push(0x4D);
                }
                /*opcode :78*/ Operator::NOR => {
                    val.push(0x4E);
                }
            }
        }

        return val;
    }

    pub fn str_op_value_bytes(val: &String) -> Vec<u8> {
        let mut val_bytes: Vec<u8> = Vec::new();
        let string_bytes = val.as_bytes();
        let n_bytes = string_bytes.len();
        let n_bytes_bytes = n_bytes.to_be_bytes();
        val_bytes.extend_from_slice(&n_bytes_bytes);
        val_bytes.extend_from_slice(&string_bytes);

        return val_bytes;
    }

    pub fn write_to_file(bytes: &Vec<u8>, file_path: &str) {
        //write the program_binary to a file
        let mut file = File::create(file_path).unwrap();
        file.write_all(&bytes).unwrap();
        file.flush().unwrap();
    }

    pub fn parse_binary(program_binary: Vec<u8>) -> Vec<Operator> {
        let mut operations: Vec<Operator> = Vec::new();
        let n_bytes = program_binary.len();
        //check for magic_bytes at start
        let magic_bytes: [u8; 4] = 0xDEADFACE_u32.to_be_bytes();
        for i in 0..4 {
            if magic_bytes[i] != program_binary[i] {
                panic!("magic_bytes not found at start of program_binary");
            }
        }

        let mut i = 4;
        for _ in 4..n_bytes - 1 {
            if i >= n_bytes {
                break;
            }
            let byte = program_binary[i];

            match byte {
                0x01 => {
                    //read next 4 bytes and treat them as a u32
                    let mut u32_bytes: [u8; 4] = [0; 4];
                    u32_bytes.copy_from_slice(&program_binary[i + 1..i + 5]);
                    i += 4;
                    let u32_val: u32 = u32::from_be_bytes(u32_bytes);

                    operations.push(Operator::PUSH(u32_val));
                }
                0x02 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    operations.push(Operator::LOAD(usize_value));
                    i += bytes_read;
                }
                0x03 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    operations.push(Operator::LOADD(usize_value));
                    i += bytes_read;
                }
                0x04 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    let (u32_value, bytes_read_2) = read_next_u32(&program_binary, i);
                    i += bytes_read_2;
                    operations.push(Operator::CONST_U(usize_value, u32_value));
                }
                0x05 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    let (f32_value, bytes_read_2) = read_next_f32(&program_binary, i);
                    i += bytes_read_2;
                    operations.push(Operator::CONST_F(usize_value, f32_value));
                }
                0x06 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    let (i32_value, bytes_read_2) = read_next_i32(&program_binary, i);
                    i += bytes_read_2;
                    operations.push(Operator::CONST_I(usize_value, i32_value));
                }
                0x07 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    let (bool, bytes_read_2) = read_next_bool(&program_binary, i);
                    i += bytes_read_2;
                    operations.push(Operator::CONST_B(usize_value, bool));
                }
                0x08 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (usize_value_2, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    operations.push(Operator::CONST_S(usize_value, usize_value_2));
                }
                0x09 => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    operations.push(Operator::LOAD_CONST(usize_value));
                }
                0x0a => {
                    operations.push(Operator::POP);
                }
                0x0b => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;
                    let (u32_value, bytes_read_2) = read_next_u32(&program_binary, i);
                    i += bytes_read_2;

                    operations.push(Operator::ALLOC(usize_value, u32_value));
                }
                0x0c => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;

                    operations.push(Operator::DEALLOC(usize_value));
                }
                0x0d => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;

                    operations.push(Operator::POPS(usize_value));
                }
                0x0e => {
                    let (usize_value, bytes_read) = read_next_usize(&program_binary, i);
                    i += bytes_read;

                    operations.push(Operator::GETLEN(usize_value));
                }
                0x0f => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    operations.push(Operator::JMP(string));
                }
                0x10 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    operations.push(Operator::JMPo(string));
                }
                0x11 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    operations.push(Operator::JMPe(string));
                }
                0x12 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    operations.push(Operator::JMPne(string));
                }
                0x13 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    let (string_length2, str_len_read2) = read_next_usize(&program_binary, i);
                    i += str_len_read2;
                    let (string2, bytes_read_22) =
                        read_next_string(&program_binary, i, string_length2);
                    i += bytes_read_22;
                    operations.push(Operator::JMPs(string, string2));
                }
                0x14 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    let (usize, usize_len) = read_next_usize(&program_binary, i);
                    i += usize_len;
                    operations.push(Operator::JMP_DEF(string, usize));
                }
                0x15 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);

                    i += bytes_read_2;
                    operations.push(Operator::LABEL(string));
                }
                0x16 => {
                    let (usize1, usize1bytes) = read_next_usize(&program_binary, i);
                    i += usize1bytes;
                    let (usize2, usize2bytes) = read_next_usize(&program_binary, i);
                    i += usize2bytes;
                    operations.push(Operator::SYSCALL(usize1, usize2));
                }
                0x17 => {
                    let (usize1, usize1bytes) = read_next_usize(&program_binary, i);
                    i += usize1bytes;
                    operations.push(Operator::SYSCALLD(usize1));
                }
                0x18 => {
                    operations.push(Operator::EXCEPT_THROW);
                }
                0x19 => {
                    let (string_length, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (string, bytes_read_2) =
                        read_next_string(&program_binary, i, string_length);
                    i += bytes_read_2;
                    operations.push(Operator::EXCEPT_CATCH(string));
                }
                0x1A => {
                    operations.push(Operator::RET);
                }
                0x1B => {
                    operations.push(Operator::EMIT);
                }
                0x1C => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    operations.push(Operator::EMITS(usize_val));
                }
                0x1D => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    operations.push(Operator::EMITW(usize_val));
                }
                0x1E => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    operations.push(Operator::EMITD(usize_val));
                }
                0x1F => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    operations.push(Operator::GETBYTELEN(usize_val));
                }
                0x20 => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (usize_val2, str_len_read2) = read_next_usize(&program_binary, i);
                    i += str_len_read2;
                    operations.push(Operator::GETBYTE(usize_val, usize_val2));
                }
                0x21 => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (usize_val2, str_len_read2) = read_next_usize(&program_binary, i);
                    i += str_len_read2;
                    operations.push(Operator::GETWORD(usize_val, usize_val2));
                }
                0x22 => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (usize_val2, str_len_read2) = read_next_usize(&program_binary, i);
                    i += str_len_read2;
                    i += 1;
                    let next_byte = program_binary[i];

                    operations.push(Operator::SETBYTE(usize_val, usize_val2, next_byte));
                }
                0x23 => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    let (usize_val2, str_len_read2) = read_next_usize(&program_binary, i);
                    i += str_len_read2;
                    let (word, word_length) = read_next_u32(&program_binary, i);
                    i += word_length;
                    operations.push(Operator::SETWORD(usize_val, usize_val2, word));
                }
                0x24 => {
                    let (usize_val, str_len_read) = read_next_usize(&program_binary, i);
                    i += str_len_read;
                    operations.push(Operator::DUPO(usize_val));
                }
                0x25 => {
                    operations.push(Operator::DUP);
                }
                0x26 => {
                    operations.push(Operator::SWAP);
                }
                0x27 => {
                    operations.push(Operator::ADDu);
                }
                0x28 => {
                    operations.push(Operator::ADDi);
                }
                0x29 => {
                    operations.push(Operator::ADDfi);
                }
                0x2A => {
                    operations.push(Operator::ADDif);
                }
                0x2B => {
                    operations.push(Operator::ADDf);
                }
                0x2C => {
                    operations.push(Operator::SUBu);
                }
                0x2D => {
                    operations.push(Operator::SUBi);
                }
                0x2E => {
                    operations.push(Operator::SUBfi);
                }
                0x2F => {
                    operations.push(Operator::SUBif);
                }
                0x30 => {
                    operations.push(Operator::SUBf);
                }
                0x31 => {
                    operations.push(Operator::MULu);
                }
                0x32 => {
                    operations.push(Operator::MULi);
                }
                0x33 => {
                    operations.push(Operator::MULfi);
                }
                0x34 => {
                    operations.push(Operator::MULif);
                }
                0x35 => {
                    operations.push(Operator::MULf);
                }
                0x36 => {
                    operations.push(Operator::DIVu);
                }
                0x37 => {
                    operations.push(Operator::DIVi);
                }
                0x38 => {
                    operations.push(Operator::DIVfi);
                }
                0x39 => {
                    operations.push(Operator::DIVif);
                }
                0x3A => {
                    operations.push(Operator::DIVf);
                }
                0x3B => {
                    operations.push(Operator::MODu);
                }
                0x3C => {
                    operations.push(Operator::MODi);
                }
                0x3D => {
                    operations.push(Operator::MODfi);
                }
                0x3E => {
                    operations.push(Operator::MODif);
                }
                0x3F => {
                    operations.push(Operator::MODf);
                }
                0x40 => {
                    operations.push(Operator::ROR);
                }
                0x41 => {
                    operations.push(Operator::ROL);
                }
                0x42 => {
                    operations.push(Operator::LSR);
                }
                0x43 => {
                    operations.push(Operator::ASR);
                }
                0x44 => {
                    operations.push(Operator::LSL);
                }
                0x45 => {
                    operations.push(Operator::ASL);
                }
                0x46 => {
                    operations.push(Operator::NEG);
                }
                0x47 => {
                    operations.push(Operator::AND);
                }
                0x48 => {
                    operations.push(Operator::XOR);
                }
                0x49 => {
                    operations.push(Operator::NAND);
                }
                0x4A => {
                    operations.push(Operator::CNT);
                }
                0x4B => {
                    operations.push(Operator::CMP);
                }
                0x4c => {
                    operations.push(Operator::JMP_SCAN)
                }
                0x4D => {
                    operations.push(Operator::OR);
                }
                0x4E => {
                    operations.push(Operator::NOR);
                }
                _ => {
                    panic!("Unknown opcode: {}", byte);
                }
            }

            i += 1;
        }

        operations
    }

    fn read_next_usize(program_binary: &Vec<u8>, i: usize) -> (usize, usize) {
        let read_next_n = mem::size_of::<usize>();
        let mut next_n_bytes: Vec<u8> = Vec::new();
        let mut _i = i;
        for _ in 0..read_next_n {
            _i = _i + 1;
            next_n_bytes.push(program_binary[_i]);
        }
        let value = usize::from_be_bytes(next_n_bytes.as_slice().try_into().unwrap());

        (value, read_next_n)
    }

    fn read_next_u32(program_binary: &Vec<u8>, i: usize) -> (u32, usize) {
        let read_next_n = 4;
        let mut next_n_bytes: Vec<u8> = Vec::new();
        let mut _i = i;
        for _ in 0..read_next_n {
            _i = _i + 1;
            next_n_bytes.push(program_binary[_i]);
        }
        let value = u32::from_be_bytes(next_n_bytes.as_slice().try_into().unwrap());

        (value, read_next_n)
    }

    fn read_next_f32(program_binary: &Vec<u8>, i: usize) -> (f32, usize) {
        let read_next_n = 4;
        let mut next_n_bytes: Vec<u8> = Vec::new();
        let mut _i = i;
        for _ in 0..read_next_n {
            _i = _i + 1;
            next_n_bytes.push(program_binary[_i]);
        }
        let value = f32::from_be_bytes(next_n_bytes.as_slice().try_into().unwrap());

        (value, read_next_n)
    }

    fn read_next_i32(program_binary: &Vec<u8>, i: usize) -> (i32, usize) {
        let read_next_n = 4;
        let mut next_n_bytes: Vec<u8> = Vec::new();
        let mut _i = i;
        for _ in 0..read_next_n {
            _i = _i + 1;
            next_n_bytes.push(program_binary[_i]);
        }
        let value = i32::from_be_bytes(next_n_bytes.as_slice().try_into().unwrap());

        (value, read_next_n)
    }

    fn read_next_bool(program_binary: &Vec<u8>, i: usize) -> (bool, usize) {
        let next_byte = program_binary[i + 1];
        let value: bool = match next_byte {
            0x00 => false,
            0xff => true,
            _ => panic!("Invalid bool value: {}", next_byte),
        };

        return (value, 1);
    }

    fn read_next_string(program_binary: &Vec<u8>, i: usize, string_size: usize) -> (String, usize) {
        let mut next_n_bytes: Vec<u8> = Vec::new();
        let mut _i = i;
        for _ in 0..string_size {
            _i = _i + 1;
            next_n_bytes.push(program_binary[_i]);
        }
        //convert each byte to a char
        let mut string_chars: Vec<char> = Vec::new();
        for byte in next_n_bytes.iter() {
            string_chars.push(*byte as char);
        }
        let value = String::from_utf8_lossy(&*next_n_bytes).to_string();

        let len = value.len();
        (value, len)
    }
}
