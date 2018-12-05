use super::*;

const INPUT: &str = include_str!("../../input/2018/day1.txt");

#[test]
fn parse_input() {
    let input = "1\n -2\n3 \n1";

    let answer = parse(input);

    assert_eq!(answer, vec![1, -2, 3, 1]);
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, -2, 3, 1];

        let answer = accumulate(&input);

        assert_eq!(answer, 3);
    }

    #[test]
    fn example2() {
        let input = vec![1, 1, 1];

        let answer = accumulate(&input);

        assert_eq!(answer, 3);
    }

    #[test]
    fn example3() {
        let input = vec![1, 1, -2];

        let answer = accumulate(&input);

        assert_eq!(answer, 0);
    }

    #[test]
    fn example4() {
        let input = vec![-1, -2, -3];

        let answer = accumulate(&input);

        assert_eq!(answer, -6);
    }

    #[test]
    fn answer() {
        let answer = accumulate(&parse(INPUT));

        assert_eq!(answer, 445);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, -2, 3, 1];

        let answer = calibrate(&input);

        assert_eq!(answer, 2);
    }

    #[test]
    fn example2() {
        let input = vec![1, -1];

        let answer = calibrate(&input);

        assert_eq!(answer, 0);
    }

    #[test]
    fn example3() {
        let input = vec![3, 3, 4, -2, -4];

        let answer = calibrate(&input);

        assert_eq!(answer, 10);
    }

    #[test]
    fn example4() {
        let input = vec![-6, 3, 8, 5, -6];

        let answer = calibrate(&input);

        assert_eq!(answer, 5);
    }

    #[test]
    fn example5() {
        let input = vec![7, 7, -2, -7, -4];

        let answer = calibrate(&input);

        assert_eq!(answer, 14);
    }

    #[test]
    fn answer() {
        let answer = calibrate(&parse(INPUT));

        assert_eq!(answer, 219);
    }
}
