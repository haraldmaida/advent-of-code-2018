use super::*;

const INPUT: &str = include_str!("../../input/2018/day19.txt");

const EXAMPLE1_INPUT: &str = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";

mod parse {
    use super::*;

    #[test]
    fn example1_input() {
        let program = parse(EXAMPLE1_INPUT).unwrap();

        assert_eq!(
            program,
            Program::new(
                0,
                vec![
                    Instruction::new(SetI, 5, 0, 1),
                    Instruction::new(SetI, 6, 0, 2),
                    Instruction::new(AddI, 0, 1, 0),
                    Instruction::new(AddR, 1, 2, 3),
                    Instruction::new(SetR, 1, 0, 0),
                    Instruction::new(SetI, 8, 0, 4),
                    Instruction::new(SetI, 9, 0, 5),
                ]
            )
        );
    }

    #[test]
    fn input() {
        let program = parse(INPUT).unwrap();

        assert_eq!(program.to_string(), INPUT);
    }
}

mod program {
    use super::*;

    #[test]
    fn display_example1() {
        let program = parse(EXAMPLE1_INPUT).unwrap();

        assert_eq!(program.to_string(), EXAMPLE1_INPUT);
    }
}

mod interpreter {
    use super::*;

    #[test]
    fn example1() {
        let program = parse(EXAMPLE1_INPUT).unwrap();

        let mut interpreter = Interpreter::new(program.ip_reg);
        let mut register = Register::default();
        interpreter
            .run(program.instructions(), &mut register)
            .unwrap();

        assert_eq!(register, Register::from([6, 5, 6, 0, 0, 9]));
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let program = parse(EXAMPLE1_INPUT).unwrap();

        let answer = run_background_process(&program);

        assert_eq!(answer, 6);
    }

    #[test]
    fn answer() {
        let program = parse(INPUT).unwrap();

        let answer = run_background_process(&program);

        assert_eq!(answer, 1056);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn answer() {
        let program = parse(INPUT).unwrap();

        let answer = run_background_process_2(&program);

        assert_eq!(answer, 10_915_260);
    }
}
