use super::*;

const INPUT: &str = include_str!("../../input/2018/day14.txt");

#[test]
fn parse_input() {
    assert_eq!(parse(INPUT), 110201);
}

mod score_seq {
    use super::*;

    #[test]
    fn fmt_display() {
        assert_eq!(
            ScoreSeq([5, 1, 5, 8, 9, 1, 6, 7, 7, 9]).to_string(),
            "[5, 1, 5, 8, 9, 1, 6, 7, 7, 9]"
        );
    }
}

mod scoreboard {
    use super::*;

    #[test]
    fn fmt_display() {
        assert_eq!(
            Scoreboard::new(vec![
                3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2
            ])
            .to_string(),
            "3  7  1  0  1  0  1  2  4  5  1  5  8  9  1  6  7  7  9  2"
        );
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let num_recipes = 9;

        let answer = score_seq_after_num_recipes(&num_recipes);

        assert_eq!(answer, ScoreSeq([5, 1, 5, 8, 9, 1, 6, 7, 7, 9]));
    }

    #[test]
    fn example2() {
        let num_recipes = 5;

        let answer = score_seq_after_num_recipes(&num_recipes);

        assert_eq!(answer, ScoreSeq([0, 1, 2, 4, 5, 1, 5, 8, 9, 1]));
    }

    #[test]
    fn example3() {
        let num_recipes = 18;

        let answer = score_seq_after_num_recipes(&num_recipes);

        assert_eq!(answer, ScoreSeq([9, 2, 5, 1, 0, 7, 1, 0, 8, 5]));
    }

    #[test]
    fn example4() {
        let num_recipes = 2018;

        let answer = score_seq_after_num_recipes(&num_recipes);

        assert_eq!(answer, ScoreSeq([5, 9, 4, 1, 4, 2, 9, 8, 8, 2]));
    }

    #[test]
    fn answer() {
        let num_recipes = parse(INPUT);

        let answer = score_seq_after_num_recipes(&num_recipes);

        assert_eq!(answer, ScoreSeq([6, 1, 0, 7, 1, 0, 1, 5, 4, 4]));
    }
}
