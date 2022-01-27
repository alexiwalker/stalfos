pub mod ops {
    #[allow(non_camel_case_types)]
    pub enum Operator {
        //Core operators: push and pop onto / off of stack
        PUSH(u32),
        LOAD(usize),

        CONST_U(usize, u32),
        CONST_F(usize, f32),
        CONST_I(usize, i32),
        CONST_B(usize, bool),
        LOAD_CONST(usize),

        //preset ptr id, n bytes
        POP,
        ALLOC(usize, u32),
        DEALLOC(usize),
        POPS(usize),
        GETLEN(usize),
        GETBYTELEN(usize),
        GETBYTE(usize,usize),
        GETWORD(usize,usize),
        SETBYTE(usize,usize,u8),
        SETWORD(usize,usize,u32),




        //i = int, f= float, fi = float<op>int, if = int<op>float
        ADDi,        ADDfi,        ADDif,        ADDf,
        SUBi,        SUBfi,        SUBif,        SUBf,
        MULi,        MULfi,        MULif,        MULf,
        DIVi,        DIVfi,        DIVif,        DIVf,
        MODi,        MODfi,        MODif,        MODf,

        //bitwise ops
        NEG,
        //invert all bits
        AND,
        XOR,
        NAND,
        CNT, //popcnt, get number of bits set

        CMP,
        JMP(String),
        //compare all bits. (lop,rop)-> *u32 lop - *u32 rop == 0. cast to uint, sub, compare to 0
        JMPe(String),
        //jump if cmp bit is 1
        JMPne(String), //jump if cmp bit is 0
        JMPs(String,String),
        // This is a compile time operation to determine the location for the jumps.
        // Therefore, all JUMP_DEFS are placed at the start of the binary
        // This is done to ensure that the jumps are always available prior to the code being executed.
        // This maps a string label to a u32 offset.that represents the program counter
        JMP_DEF(String, usize),

        LABEL(String),

        SYSCALL(usize,usize), //system call. left op is syscall id, right op is number of args


        EXCEPT_THROW, //throw exception
        EXCEPT_CATCH(String), //catch exception - jump to different location. When proceeding normally (not in a stack unwind) this is a noop

        RET
    }
}