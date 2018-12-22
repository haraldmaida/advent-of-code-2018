use super::*;

const INPUT: &str = include_str!("../../input/2018/day18.txt");

const EXAMPLE1_INITIAL: &str = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
";

const EXAMPLE1_AFTER_1_MINUTE: &str = "\
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.
";

const EXAMPLE1_AFTER_2_MINUTES: &str = "\
.......#..
......|#..
.|.|||....
..##|||..#
..###|||#|
...#|||||.
|||||||||.
||||||||||
||||||||||
.|||||||||
";

const EXAMPLE1_AFTER_3_MINUTES: &str = "\
.......#..
....|||#..
.|.||||...
..###|||.#
...##|||#|
.||##|||||
||||||||||
||||||||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_4_MINUTES: &str = "\
.....|.#..
...||||#..
.|.#||||..
..###||||#
...###||#|
|||##|||||
||||||||||
||||||||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_5_MINUTES: &str = "\
....|||#..
...||||#..
.|.##||||.
..####|||#
.|.###||#|
|||###||||
||||||||||
||||||||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_6_MINUTES: &str = "\
...||||#..
...||||#..
.|.###|||.
..#.##|||#
|||#.##|#|
|||###||||
||||#|||||
||||||||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_7_MINUTES: &str = "\
...||||#..
..||#|##..
.|.####||.
||#..##||#
||##.##|#|
|||####|||
|||###||||
||||||||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_8_MINUTES: &str = "\
..||||##..
..|#####..
|||#####|.
||#...##|#
||##..###|
||##.###||
|||####|||
||||#|||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_9_MINUTES: &str = "\
..||###...
.||#####..
||##...##.
||#....###
|##....##|
||##..###|
||######||
|||###||||
||||||||||
||||||||||
";

const EXAMPLE1_AFTER_10_MINUTES: &str = "\
.||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||
";

mod parse {
    use super::*;

    #[test]
    fn example1_initial() {
        let area = parse(EXAMPLE1_INITIAL);

        assert_eq!(area.to_string(), EXAMPLE1_INITIAL);
    }

    #[test]
    fn input() {
        let area = parse(INPUT);

        assert_eq!(area.to_string(), INPUT);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1_after_1_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(1);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_1_MINUTE);
    }

    #[test]
    fn example1_after_2_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(2);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_2_MINUTES);
    }

    #[test]
    fn example1_after_3_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(3);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_3_MINUTES);
    }

    #[test]
    fn example1_after_4_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(4);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_4_MINUTES);
    }

    #[test]
    fn example1_after_5_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(5);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_5_MINUTES);
    }

    #[test]
    fn example1_after_6_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(6);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_6_MINUTES);
    }

    #[test]
    fn example1_after_7_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(7);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_7_MINUTES);
    }

    #[test]
    fn example1_after_8_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(8);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_8_MINUTES);
    }

    #[test]
    fn example1_after_9_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(9);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_9_MINUTES);
    }

    #[test]
    fn example1_after_10_minute() {
        let area = parse(EXAMPLE1_INITIAL);

        let mutated = area.nth_generation(10);

        assert_eq!(mutated.to_string(), EXAMPLE1_AFTER_10_MINUTES);
    }

    #[test]
    fn example1() {
        let area = parse(EXAMPLE1_INITIAL);

        let answer = total_resource_value_after_10_minutes(&area);

        assert_eq!(answer, 1147);
    }

    #[test]
    fn answer() {
        let area = parse(INPUT);

        let answer = total_resource_value_after_10_minutes(&area);

        assert_eq!(answer, 620_624);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn answer() {
        let area = parse(INPUT);

        let answer = total_resource_value_after_1_000_000_000_minutes(&area);

        assert_eq!(answer, 169_234);
    }
}
