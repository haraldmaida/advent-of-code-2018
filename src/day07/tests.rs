use super::*;

const INPUT: &str = include_str!("../../input/2018/day7.txt");

const EXAMPLE1_INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

mod parse_input {
    use super::*;

    #[test]
    fn example1() {
        let instructions = parse(EXAMPLE1_INPUT);

        assert_eq!(instructions.len(), 6);
        assert_eq!(
            instructions.prerequisites(&'A'),
            &HashSet::from_iter(vec!['C'])
        );
        assert_eq!(
            instructions.prerequisites(&'B'),
            &HashSet::from_iter(vec!['A'])
        );
        assert_eq!(
            instructions.prerequisites(&'C'),
            &HashSet::from_iter(vec![])
        );
        assert_eq!(
            instructions.prerequisites(&'D'),
            &HashSet::from_iter(vec!['A'])
        );
        assert_eq!(
            instructions.prerequisites(&'E'),
            &HashSet::from_iter(vec!['B', 'D', 'F'])
        );
        assert_eq!(
            instructions.prerequisites(&'F'),
            &HashSet::from_iter(vec!['C'])
        );
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let execution_plan = parse(EXAMPLE1_INPUT);

        let answer = String::from_iter(execution_plan.in_order());

        assert_eq!(answer, "CABDFE");
    }

    #[test]
    fn answer() {
        let answer = execution_order(&parse(INPUT));

        assert_eq!(answer, "BHRTWCYSELPUVZAOIJKGMFQDXN");
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let instruction_set = InstructionSet::new(Duration::from_sec(0));
        let execution_plan = parse(EXAMPLE1_INPUT);

        let answer = execution_plan.execution_time(2, instruction_set);

        assert_eq!(answer, Duration::from_sec(15));
    }

    #[test]
    fn answer() {
        let answer = execution_time(&parse(INPUT));

        assert_eq!(answer, Duration::from_sec(959));
    }
}
