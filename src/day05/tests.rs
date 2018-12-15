use super::*;

const INPUT: &str = include_str!("../../input/2018/day5.txt");

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = "aA";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "");
    }

    #[test]
    fn example2() {
        let input = "abBA";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "");
    }

    #[test]
    fn example3() {
        let input = "aabAAB";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "aabAAB");
    }

    #[test]
    fn example4() {
        let input = "dabAcCaCBAcCcaDA";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "dabCBAcaDA");
    }

    #[test]
    fn example5() {
        let input = "abcCcd";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "abcd");
    }

    #[test]
    fn example6() {
        let input = "abcCcdD";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "abc");
    }

    #[test]
    fn example7() {
        let input = "aaBbaAB";

        let answer = reduce_polymer(input);

        assert_eq!(&answer, "aaB");
    }

    #[test]
    fn answer() {
        let answer = reduced_polymer_len(INPUT);

        assert_eq!(answer, 9348);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = "dabAcCaCBAcCcaDA";

        let answer = improve_polymer(input);

        assert_eq!(answer, ('c', "daDA".into()));
    }

    #[ignore]
    #[test]
    fn answer() {
        let answer = improved_polymer_len(INPUT);

        assert_eq!(answer, 4996)
    }
}
