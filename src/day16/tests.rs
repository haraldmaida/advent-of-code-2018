use super::*;

const INPUT: &str = include_str!("../../input/2018/day16.txt");

const EXAMPLE1_INPUT: &str = "\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";

mod parse_input {
    use super::*;

    #[test]
    fn example1() {
        let (samples, instructions) = parse(EXAMPLE1_INPUT).unwrap();

        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].to_string(), EXAMPLE1_INPUT);
        assert_eq!(instructions.len(), 0);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let samples_n_program = parse(EXAMPLE1_INPUT).unwrap();

        let answer = num_samples_behaving_like_three_or_more_opcodes(&samples_n_program);

        assert_eq!(answer, 1);
    }

    #[test]
    fn answer() {
        let samples_n_program = parse(INPUT).unwrap();

        let answer = num_samples_behaving_like_three_or_more_opcodes(&samples_n_program);

        assert_eq!(answer, 646);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn answer() {
        let samples_n_program = parse(INPUT).unwrap();

        let answer = run_program(&samples_n_program);

        assert_eq!(answer, Register::from([681, 681, 3, 0]));
    }
}
