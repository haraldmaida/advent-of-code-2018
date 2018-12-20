use super::*;

const INPUT: &str = include_str!("../../input/2018/day17.txt");

const EXAMPLE1_INPUT: &str = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";

mod parse_input {
    use super::*;

    #[test]
    fn parse_example1() {
        let scan = parse(EXAMPLE1_INPUT);
        eprintln!("{}", scan);
        assert_eq!(scan.tiles.len(), 34);
    }

    #[test]
    fn parse_input() {
        let scan = parse(INPUT);
        eprintln!("{}", scan);
        assert_eq!(scan.tiles.len(), 16721);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let scan = parse(EXAMPLE1_INPUT);

        let answer = num_tiles_flooded_by_water(&scan);

        assert_eq!(answer, 57);
    }

    #[test]
    fn answer() {
        let scan = parse(INPUT);

        let answer = num_tiles_flooded_by_water(&scan);

        assert_eq!(answer, 1523699);
    }
}
