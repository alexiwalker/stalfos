pub mod example_programs {
    use stalfos_vm::stalfos::ops::Operator;

    pub fn hello_world() -> Vec<Operator> {
        vec![
            Operator::JMP_SCAN,
            Operator::LABEL("main".to_string()),
            Operator::CONST_S(1, "hello world!".to_string()),
            Operator::LOADD(1),
            Operator::SYSCALLD(3),
            Operator::RET,
        ]
    }

    pub fn single_opcode() -> Vec<Operator> {
        vec![Operator::CONST_S(1, "hello world!".to_string())]
    }

    pub fn jmp_except_catch() -> Vec<Operator> {
        vec![
            //commented values are the program counter if not using JMP_SCAN to load labels.
            Operator::JMP_SCAN,
            // Operator::JMP_DEF("main".to_string(), 3),//0
            // Operator::JMP_DEF("print".to_string(), 16),//1
            // Operator::JMP_DEF("except".to_string(),21), //2
            //main
            Operator::LABEL("main".to_string()),          //3
            Operator::EXCEPT_CATCH("except".to_string()), //4
            Operator::PUSH(5),                            //5
            Operator::PUSH(5),                            //6
            Operator::MULi,                               //7
            Operator::POPS(1),                            //8
            Operator::LOAD(1),                            //9
            Operator::SYSCALL(1, 1),                      //10
            Operator::PUSH(25),                           //11
            Operator::LOAD(1),                            //12
            Operator::CMP,                                //13
            Operator::JMPe("print".to_string()),          //14
            Operator::RET,                                //15
            //
            //
            // print
            Operator::LABEL("print".to_string()), //16
            Operator::PUSH(42069),                //17
            Operator::SYSCALL(1, 1),              //18
            Operator::EXCEPT_THROW,               //19
            Operator::RET,                        //20
            //
            // except
            Operator::LABEL("except".to_string()), //21
            Operator::PUSH(1111),                  //22
            Operator::SYSCALL(1, 1),               //23
            Operator::SYSCALL(2, 0),               //23
        ]
    }
    /**
    0    222,    magic
    1    173,    magic
    2    250,    magic
    3    206,    magic
    4    76,     jmp_scan     <- opcode
    5    21,     label        <- opcode
    6    0,      usize 4
    7    0,      usize 4
    8    0,      usize 4
    9    0,      usize 4
    10   0,      usize 4
    11   0,      usize 4
    12   0,      usize 4
    13   4,      usize 4      <- string size prefix
    14   109,    string byte  <- string
    15   97,     string byte  <- string
    16   105,    string byte  <- string
    17   110     string byte  <- string
        */

    pub fn string_manipulation() -> Vec<Operator> {
        vec![
            Operator::JMP_SCAN,
            Operator::LABEL("main".to_string()),
            Operator::CONST_S(1,"                            ".to_string()), //empty string that you can set bytes on
            Operator::SETBYTE(1,0,0b0110_1000),
            Operator::SETBYTE(1,1,0b0110_0101),
            Operator::SETBYTE(1,2,0b0110_1100),
            Operator::SETBYTE(1,3,0b0110_1100),
            Operator::SETBYTE(1,4,0b0110_1111),
            Operator::SETBYTE(1,5,0b0010_1100),
            Operator::SETBYTE(1,6,0b0010_0000),
            Operator::SETBYTE(1,7,0b0111_0111),
            Operator::SETBYTE(1,8,0b0110_1111),
            Operator::SETBYTE(1,9,0b0111_0010),
            Operator::SETBYTE(1,10,0b0110_1100),
            Operator::SETBYTE(1,11,0b0110_0100),
            Operator::SETBYTE(1,12,0b0010_0001),
            Operator::SETBYTE(1,13,0b0010_0000),
            Operator::SETBYTE(1,14,0b0011_1010),
            Operator::SETBYTE(1,15,0b0010_1001),
            Operator::LOADD(1),
            Operator::SYSCALLD(3),
            Operator::CONST_S(2,"\"hello, world! :)\" was built by setting individual bytes. You can manipulate individual bytes in a string by index with SETBYTE. \n\nTo create a string, use CONST_S. To put it on the stack with its length, call LOADD. To print it, call SYSCALLD(3)".to_string()),
            Operator::LOADD(2),
            Operator::SYSCALLD(3),
            Operator::CONST_S(2,"Allocation names can be reused. This, and the message above, both use identifier 2. This will cause a reallocation. The previous value will be lost The next line is longer and also uses ID 2. This does not affect the resulting string".to_string()),
            Operator::LOADD(2),
            Operator::SYSCALLD(3),
            Operator::CONST_S(2,"VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING VERY LONG STRING ".to_string()),
            Operator::LOADD(2),
            Operator::SYSCALLD(3),
            Operator::RET,
        ]
    }
}
