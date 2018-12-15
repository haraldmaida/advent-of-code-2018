use super::*;

const INPUT: &str = include_str!("../../input/2018/day9.txt");

mod game_runner {
    use super::*;

    #[test]
    fn example1_step0() {
        let game = MarbleGame::new(29, 3);
        let runner = GameRunner::new(game);

        assert_eq!(runner.marbles(), &[Marble(0)]);
        assert_eq!(runner.current_marble(), Marble(0));
        assert_eq!(runner.current_player(), PlayerNr(0));
        assert_eq!(runner.score(runner.current_player()), None);
    }

    #[test]
    fn example1_step1() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(0);

        assert_eq!(result, Some(()));
        assert_eq!(runner.marbles(), &[Marble(0), Marble(1)]);
        assert_eq!(runner.current_marble(), Marble(1));
        assert_eq!(runner.current_player(), PlayerNr(1));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step2() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(1);

        assert_eq!(result, Some(()));
        assert_eq!(runner.marbles(), &[Marble(0), Marble(2), Marble(1)]);
        assert_eq!(runner.current_marble(), Marble(2));
        assert_eq!(runner.current_player(), PlayerNr(2));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step3() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(2);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[Marble(0), Marble(2), Marble(1), Marble(3)]
        );
        assert_eq!(runner.current_marble(), Marble(3));
        assert_eq!(runner.current_player(), PlayerNr(3));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step4() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(3);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[Marble(0), Marble(4), Marble(2), Marble(1), Marble(3)]
        );
        assert_eq!(runner.current_marble(), Marble(4));
        assert_eq!(runner.current_player(), PlayerNr(1));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step5() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(4);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[
                Marble(0),
                Marble(4),
                Marble(2),
                Marble(5),
                Marble(1),
                Marble(3)
            ]
        );
        assert_eq!(runner.current_marble(), Marble(5));
        assert_eq!(runner.current_player(), PlayerNr(2));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step6() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(5);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[
                Marble(0),
                Marble(4),
                Marble(2),
                Marble(5),
                Marble(1),
                Marble(6),
                Marble(3)
            ]
        );
        assert_eq!(runner.current_marble(), Marble(6));
        assert_eq!(runner.current_player(), PlayerNr(3));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step7() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(6);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[
                Marble(0),
                Marble(4),
                Marble(2),
                Marble(5),
                Marble(1),
                Marble(6),
                Marble(3),
                Marble(7),
            ]
        );
        assert_eq!(runner.current_marble(), Marble(7));
        assert_eq!(runner.current_player(), PlayerNr(1));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step22() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(21);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[
                Marble(0),
                Marble(16),
                Marble(8),
                Marble(17),
                Marble(4),
                Marble(18),
                Marble(9),
                Marble(19),
                Marble(2),
                Marble(20),
                Marble(10),
                Marble(21),
                Marble(5),
                Marble(22),
                Marble(11),
                Marble(1),
                Marble(12),
                Marble(6),
                Marble(13),
                Marble(3),
                Marble(14),
                Marble(7),
                Marble(15),
            ]
        );
        assert_eq!(runner.current_marble(), Marble(22));
        assert_eq!(runner.current_player(), PlayerNr(1));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }

    #[test]
    fn example1_step23() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(22);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[
                Marble(0),
                Marble(16),
                Marble(8),
                Marble(17),
                Marble(4),
                Marble(18),
                Marble(19),
                Marble(2),
                Marble(20),
                Marble(10),
                Marble(21),
                Marble(5),
                Marble(22),
                Marble(11),
                Marble(1),
                Marble(12),
                Marble(6),
                Marble(13),
                Marble(3),
                Marble(14),
                Marble(7),
                Marble(15),
            ]
        );
        assert_eq!(runner.current_marble(), Marble(23));
        assert_eq!(runner.current_player(), PlayerNr(2));
        assert_eq!(runner.score(runner.current_player()), Some(Score(32)));
    }

    #[test]
    fn example1_step24() {
        let game = MarbleGame::new(29, 3);
        let mut runner = GameRunner::new(game);

        let result = runner.nth(23);

        assert_eq!(result, Some(()));
        assert_eq!(
            runner.marbles(),
            &[
                Marble(0),
                Marble(16),
                Marble(8),
                Marble(17),
                Marble(4),
                Marble(18),
                Marble(19),
                Marble(2),
                Marble(24),
                Marble(20),
                Marble(10),
                Marble(21),
                Marble(5),
                Marble(22),
                Marble(11),
                Marble(1),
                Marble(12),
                Marble(6),
                Marble(13),
                Marble(3),
                Marble(14),
                Marble(7),
                Marble(15),
            ]
        );
        assert_eq!(runner.current_marble(), Marble(24));
        assert_eq!(runner.current_player(), PlayerNr(3));
        assert_eq!(runner.score(runner.current_player()), Some(Score(0)));
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let game = MarbleGame::new(25, 9);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(32));
    }

    #[test]
    fn example2() {
        let game = MarbleGame::new(1618, 10);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(8317));
    }

    #[test]
    fn example3() {
        let game = MarbleGame::new(7999, 13);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(146_373));
    }

    #[test]
    fn example4() {
        let game = MarbleGame::new(1104, 17);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(2764));
    }

    #[test]
    fn example5() {
        let game = MarbleGame::new(6111, 21);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(54_718));
    }

    #[test]
    fn example6() {
        let game = MarbleGame::new(5807, 30);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(37_305));
    }

    #[test]
    fn answer() {
        let game = parse(INPUT);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(439_635));
    }
}

mod part2 {
    use super::*;

    #[ignore]
    #[test]
    fn answer() {
        let game = parse(INPUT);

        let answer = marble_highscore(&game);

        assert_eq!(answer, Score(3_562_722_971));
    }
}
