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

const EXAMPLE2_INPUT: &str = "\
x=506, y=1..1
x=498, y=2..3
x=502, y=2..3
y=4, x=498..502
x=495, y=7..9
y=10, x=495..506
x=506, y=7..9
";

const EXAMPLE3_INPUT: &str = "\
x=486, y=4..15
x=510, y=3..15
y=16, x=486..510
x=496, y=8..10
x=501, y=8..10
y=11, x=496..501
";

const EXAMPLE4_INPUT: &str = "\
x=490, y=3..15
x=514, y=4..15
y=16, x=490..514
x=503, y=8..11
x=505, y=8..11
y=12, x=503..505
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
    fn example2() {
        let scan = parse(EXAMPLE2_INPUT);

        let answer = num_tiles_flooded_by_water(&scan);

        assert_eq!(answer, 73);
    }

    #[test]
    fn example3() {
        let scan = parse(EXAMPLE3_INPUT);

        let answer = num_tiles_flooded_by_water(&scan);

        assert_eq!(answer, 302);
    }

    #[test]
    fn example4() {
        let scan = parse(EXAMPLE4_INPUT);

        let answer = num_tiles_flooded_by_water(&scan);

        assert_eq!(answer, 303);
    }

    #[test]
    fn answer() {
        let scan = parse(INPUT);

        let answer = num_tiles_flooded_by_water(&scan);

        assert_eq!(answer, 31667);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let scan = parse(EXAMPLE1_INPUT);

        let answer = num_tiles_flooded_by_water_after_spring_has_run_dry(&scan);

        assert_eq!(answer, 29);
    }

    #[test]
    fn example2() {
        let scan = parse(EXAMPLE2_INPUT);

        let answer = num_tiles_flooded_by_water_after_spring_has_run_dry(&scan);

        assert_eq!(answer, 36);
    }

    #[test]
    fn example3() {
        let scan = parse(EXAMPLE3_INPUT);

        let answer = num_tiles_flooded_by_water_after_spring_has_run_dry(&scan);

        assert_eq!(answer, 264);
    }

    #[test]
    fn example4() {
        let scan = parse(EXAMPLE4_INPUT);

        let answer = num_tiles_flooded_by_water_after_spring_has_run_dry(&scan);

        assert_eq!(answer, 265);
    }

    #[ignore]
    #[test]
    fn answer() {
        let scan = parse(INPUT);

        let answer = num_tiles_flooded_by_water_after_spring_has_run_dry(&scan);

        assert_eq!(answer, 31667);
    }
}
