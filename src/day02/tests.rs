use super::*;

const INPUT: &str = include_str!("../../input/2018/day2.txt");

fn prepare(input: Vec<&str>) -> Vec<String> {
    input.into_iter().map(ToOwned::to_owned).collect()
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = prepare(vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]);

        let answer = checksum(&input);

        assert_eq!(answer, 12);
    }

    #[test]
    fn answer() {
        let answer = checksum(&parse(INPUT));

        assert_eq!(answer, 5681);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = prepare(vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]);

        let answer = search_prototype_boxes(&input);

        assert_eq!(answer, "fgij");
    }

    #[test]
    fn answer() {
        let answer = search_prototype_boxes(&parse(INPUT));

        assert_eq!(answer, "uqyoeizfvmbistpkgnocjtwld");
    }
}
