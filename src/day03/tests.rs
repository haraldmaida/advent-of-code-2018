use super::*;

const INPUT: &str = include_str!("../../input/2018/day3.txt");

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 7,
            },
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 7,
                height: 4,
            },
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            },
        ];

        let answer = overlapping_area(&input);

        assert_eq!(answer, 4);
    }

    #[test]
    fn example2() {
        let input = vec![
            Claim {
                id: 1,
                left: 3,
                top: 1,
                width: 4,
                height: 7,
            },
            Claim {
                id: 2,
                left: 1,
                top: 3,
                width: 7,
                height: 4,
            },
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            },
        ];

        let answer = overlapping_area(&input);

        assert_eq!(answer, 16);
    }

    #[test]
    fn answer() {
        let answer = overlapping_area(&parse(INPUT));

        assert_eq!(answer, 119_572);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            },
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 4,
                height: 4,
            },
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            },
        ];

        let answer = non_overlapping_claims(&input);

        assert_eq!(answer, 3);
    }

    #[test]
    fn answer() {
        let answer = non_overlapping_claims(&parse(INPUT));

        assert_eq!(answer, 775);
    }
}
