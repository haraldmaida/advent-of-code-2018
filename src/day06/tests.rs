use super::*;

const INPUT: &str = include_str!("../../input/2018/day6.txt");

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 8, y: 9 },
        ];

        let answer = largest_area(&input);

        assert_eq!(answer, (Point { x: 5, y: 5 }, 17));
    }

    #[test]
    fn example2() {
        let input = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 8, y: 9 },
        ];

        let answer = largest_area(&input);

        assert_eq!(answer, (Point { x: 3, y: 4 }, 9));
    }

    #[test]
    fn answer() {
        let answer = solve_part1(&parse(INPUT));

        assert_eq!(answer, 4398);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 8, y: 9 },
        ];

        let answer = area_within_distance(Distance(32), &input);

        assert_eq!(answer, 16);
    }

    #[test]
    fn answer() {
        let answer = solve_part2(&parse(INPUT));

        assert_eq!(answer, 39560);
    }
}
