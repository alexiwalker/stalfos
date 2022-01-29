pub mod ops {
    #[allow(non_camel_case_types)]
    pub enum Operator {
        //Core operators: push and pop onto / off of stack
        PUSH(u32),
        // LOAD only gets the first word (4bytes) of the allocation
        // this means it is only suitable for small allocations
        // use LOADD for larger allocations
        LOAD(usize),

        // LOADD (load dynamic) gets the whole allocation, most significant bytes on top. topmost
        // value on stack after this call will be the size of the allocation below
        LOADD(usize),

        CONST_U(usize, u32),
        CONST_F(usize, f32),
        CONST_I(usize, i32),
        CONST_B(usize, bool),
        CONST_S(usize, String),
        LOAD_CONST(usize),

        //preset ptr id, n bytes
        POP,
        ALLOC(usize, u32),
        DEALLOC(usize),
        POPS(usize),
        GETLEN(usize),     //number of words
        GETBYTELEN(usize), //number of bytes
        GETBYTE(usize, usize),
        GETWORD(usize, usize),
        SETBYTE(usize, usize, u8),
        SETWORD(usize, usize, u32),

        DUP,         //duplicate topmost value on stack
        DUPO(usize), //duplicate value on stack at offset
        SWAP,        //swap topmost two values on stack

        //i = int, f= float, fi = float<op>int, if = int<op>float
        ADDu,
        ADDi,
        ADDfi,
        ADDif,
        ADDf,
        SUBu,
        SUBi,
        SUBfi,
        SUBif,
        SUBf,
        MULu,
        MULi,
        MULfi,
        MULif,
        MULf,
        DIVu,
        DIVi,
        DIVfi,
        DIVif,
        DIVf,
        MODu,
        MODi,
        MODfi,
        MODif,
        MODf,

        ROR, //rotate right
        ROL, //rotate left
        LSR, //shift right
        ASR, //arithmetic shift right
        LSL, //shift left
        ASL, //arithmetic shift left

        //bitwise ops
        //invert all bits
        NEG,
        AND,
        XOR,
        OR,
        NOR,
        NAND,
        CNT, //popcnt, get number of bits set
        CMP,
        JMP_SCAN, // scans through the program for all LABELS and adds them (and their addresses) to the jmp_label map. may be slow on large programs
        JMP(String),
        JMPo(String), //jmp if overflow
        //compare all bits. (lop,rop)-> *u32 lop - *u32 rop == 0. cast to uint, sub, compare to 0
        JMPe(String),
        //jump if cmp bit is 1
        JMPne(String), //jump if cmp bit is 0

        //jumps to left is stack pop is 0, right otherwise.
        JMPs(String, String),

        // This should be a compile time operation to determine the location for the jumps.
        // Therefore, all JUMP_DEFS are placed at the start of the binary
        // This is done to ensure that the jumps are available prior to the code being executed.
        // without having to scan the entire binary for jumps.
        // This maps a string label to a u32 offset.that represents the program counter
        JMP_DEF(String, usize),

        // encountering a label without jumping causes a panic
        // JMP_SCAN will find all locations of labels and add them to the map
        LABEL(String),

        SYSCALL(usize, usize), //system call. left op is syscall id, right op is number of args
        SYSCALLD(usize), //system call. op is syscall id. pop top value off stack, reads as u32. then pops that many off stack as args

        EXCEPT_THROW,         //throw exception
        EXCEPT_CATCH(String), //catch exception - jump to different location. When proceeding normally (not in a stack unwind) this is a noop

        RET, //returns to the location that was last jumped FROM.

        EMIT,         //pop top value off stack, emit it to output stream
        EMITS(usize), //emit a string to the output stream
        EMITW(usize), //emit a word to the output stream
        EMITD(usize), //emits an unknown number of words to the output stream. The number of words is the top value on the stack.
    }
}
