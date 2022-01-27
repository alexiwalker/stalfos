pub mod example_programs {
    use stalfos_vm::stalfos::ops::Operator;

    pub fn default_example_program() -> Vec<Operator> {
        vec![
            Operator::JMP_DEF("main".to_string(), 3),//0
            Operator::JMP_DEF("print".to_string(), 16),//1
            Operator::JMP_DEF("except".to_string(),21), //2
            //main
            Operator::LABEL("main".to_string()),//3
            Operator::EXCEPT_CATCH("except".to_string()),//4
            Operator::PUSH(5),//5
            Operator::PUSH(5),//6
            Operator::MULi,//7
            Operator::POPS(1),//8
            Operator::LOAD(1),//9
            Operator::SYSCALL(1,1),//10
            Operator::PUSH(25),//11
            Operator::LOAD(1),//12
            Operator::CMP,//13
            Operator::JMPe("print".to_string()),//14
            Operator::RET,//15


            //print
            Operator::LABEL("print".to_string()),//16
            Operator::PUSH(42069),//17
            Operator::SYSCALL(1,1),//18
            Operator::EXCEPT_THROW,//19
            Operator::RET,//20

            //except
            Operator::LABEL("except".to_string()),//21
            Operator::PUSH(1111),//22
            Operator::SYSCALL(1,1),//23
            Operator::SYSCALL(2,0),//23

        ]
    }
}
