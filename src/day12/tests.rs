use super::*;

const INPUT: &str = include_str!("../../input/2018/day12.txt");

const EXAMPLE1_INPUT: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

mod plantation {
    use super::*;

    #[test]
    fn parse_example1() {
        let plantation = parse(EXAMPLE1_INPUT);

        assert_eq!(
            plantation,
            Plantation::new(
                vec![
                    true, false, false, true, false, true, false, false, true, true, false, false,
                    false, false, false, false, true, true, true, false, false, false, true, true,
                    true
                ],
                vec![
                    BreedRule::new(vec![false, false, false, true, true], true),
                    BreedRule::new(vec![false, false, true, false, false], true),
                    BreedRule::new(vec![false, true, false, false, false], true),
                    BreedRule::new(vec![false, true, false, true, false], true),
                    BreedRule::new(vec![false, true, false, true, true], true),
                    BreedRule::new(vec![false, true, true, false, false], true),
                    BreedRule::new(vec![false, true, true, true, true], true),
                    BreedRule::new(vec![true, false, true, false, true], true),
                    BreedRule::new(vec![true, false, true, true, true], true),
                    BreedRule::new(vec![true, true, false, true, false], true),
                    BreedRule::new(vec![true, true, false, true, true], true),
                    BreedRule::new(vec![true, true, true, false, false], true),
                    BreedRule::new(vec![true, true, true, false, true], true),
                    BreedRule::new(vec![true, true, true, true, false], true),
                ]
            )
        );
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let plantation = parse(EXAMPLE1_INPUT);

        let (evolved, offset) = evolve_n_generations(&plantation, 20);

        assert_eq!(
            evolved.plants.to_string(),
            ".....#....##....#####...#######....#.#..##....."
        );
        assert_eq!(offset, -7);
    }

    #[test]
    fn example1_answer() {
        let plantation = parse(EXAMPLE1_INPUT);

        let answer = sum_of_pot_numbers_after_20_generations(&plantation);

        assert_eq!(answer, 325);
    }

    #[test]
    fn answer() {
        let plantation = parse(INPUT);

        let answer = sum_of_pot_numbers_after_20_generations(&plantation);

        assert_eq!(answer, 3217);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn answer() {
        let plantation = parse(INPUT);

        let answer = sum_of_pot_numbers_after_50_000_000_000_generations(&plantation);

        assert_eq!(answer, 4_000_000_000_866);
    }
}
