use super::*;

const INPUT: &str = include_str!("../../input/2018/day15.txt");

const EXAMPLE1_INPUT: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
";

const EXAMPLE1_1ST_ROUND: &str = "\
#######
#..G..#
#...EG#
#.#G#G#
#...#E#
#.....#
#######
";

const EXAMPLE1_2ND_ROUND: &str = "\
#######
#...G.#
#..GEG#
#.#.#G#
#...#E#
#.....#
#######
";

const EXAMPLE1_22ND_ROUND: &str = "\
#######
#...G.#
#..GEG#
#.#.#G#
#...#E#
#.....#
#######
";

const EXAMPLE1_23RD_ROUND: &str = "\
#######
#...G.#
#..G.G#
#.#.#G#
#...#E#
#.....#
#######
";

const EXAMPLE1_24TH_ROUND: &str = "\
#######
#..G..#
#...G.#
#.#G#G#
#...#E#
#.....#
#######
";

const EXAMPLE1_FINAL: &str = "\
#######
#G....#
#.G...#
#.#.#G#
#...#.#
#....G#
#######
";

const EXAMPLE2_INPUT: &str = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
";

const EXAMPLE2_FINAL: &str = "\
#######
#...#E#
#E#...#
#.E##.#
#E..#E#
#.....#
#######
";

const EXAMPLE3_INPUT: &str = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
";

const EXAMPLE3_FINAL: &str = "\
#######
#.E.E.#
#.#E..#
#E.##.#
#.E.#.#
#...#.#
#######
";

const EXAMPLE4_INPUT: &str = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
";

const EXAMPLE4_FINAL: &str = "\
#######
#G.G#.#
#.#G..#
#..#..#
#...#G#
#...G.#
#######
";

const EXAMPLE5_INPUT: &str = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
";

const EXAMPLE5_FINAL: &str = "\
#######
#.....#
#.#G..#
#.###.#
#.#.#.#
#G.G#G#
#######
";

const EXAMPLE6_INPUT: &str = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
";

const EXAMPLE6_FINAL: &str = "\
#########
#.G.....#
#G.G#...#
#.G##...#
#...##..#
#.G.#...#
#.......#
#.......#
#########
";

mod parse {
    use super::*;

    #[test]
    fn example1() {
        let combat_map = parse(EXAMPLE1_INPUT);

        assert_eq!(combat_map.to_string(), EXAMPLE1_INPUT);
    }

    #[test]
    fn example2() {
        let combat_map = parse(EXAMPLE2_INPUT);

        assert_eq!(combat_map.to_string(), EXAMPLE2_INPUT);
    }

    #[test]
    fn example3() {
        let combat_map = parse(EXAMPLE3_INPUT);

        assert_eq!(combat_map.to_string(), EXAMPLE3_INPUT);
    }

    #[test]
    fn example4() {
        let combat_map = parse(EXAMPLE4_INPUT);

        assert_eq!(combat_map.to_string(), EXAMPLE4_INPUT);
    }

    #[test]
    fn example5() {
        let combat_map = parse(EXAMPLE5_INPUT);

        assert_eq!(combat_map.to_string(), EXAMPLE5_INPUT);
    }

    #[test]
    fn example6() {
        let combat_map = parse(EXAMPLE6_INPUT);

        assert_eq!(combat_map.to_string(), EXAMPLE6_INPUT);
    }

    #[test]
    fn input() {
        let combat_map = parse(INPUT);

        assert_eq!(combat_map.to_string(), INPUT);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1_1st_round() {
        let mut combat = parse(EXAMPLE1_INPUT);

        let _ = combat.n_fights(1);

        assert_eq!(combat.to_string(), EXAMPLE1_1ST_ROUND);
        assert_eq!(
            combat.elf(Position::new(4, 2)).map(|elf| elf.hit_points),
            Some(HitPoints(197))
        );
    }

    #[test]
    fn example1_2nd_round() {
        let mut combat = parse(EXAMPLE1_INPUT);

        let _ = combat.n_fights(2);

        assert_eq!(combat.to_string(), EXAMPLE1_2ND_ROUND);
        assert_eq!(
            combat.elf(Position::new(4, 2)).map(|elf| elf.hit_points),
            Some(HitPoints(188))
        );
    }

    #[test]
    fn example1_22nd_round() {
        let mut combat = parse(EXAMPLE1_INPUT);

        let _ = combat.n_fights(22);

        assert_eq!(combat.to_string(), EXAMPLE1_22ND_ROUND);
        assert_eq!(
            combat.elf(Position::new(4, 2)).map(|elf| elf.hit_points),
            Some(HitPoints(8))
        );
    }

    #[test]
    fn example1_23nd_round() {
        let mut combat = parse(EXAMPLE1_INPUT);

        let _ = combat.n_fights(23);

        assert_eq!(combat.to_string(), EXAMPLE1_23RD_ROUND);
        assert_eq!(
            combat.elf(Position::new(4, 2)).map(|elf| elf.hit_points),
            None
        );
    }

    #[test]
    fn example1_24th_round() {
        let mut combat = parse(EXAMPLE1_INPUT);

        let _ = combat.n_fights(24);

        assert_eq!(combat.to_string(), EXAMPLE1_24TH_ROUND);
        assert_eq!(
            combat.elf(Position::new(5, 4)).map(|elf| elf.hit_points),
            Some(HitPoints(128))
        );
    }

    #[test]
    fn example1_final() {
        let mut combat = parse(EXAMPLE1_INPUT);

        let result = combat.fight();

        assert_eq!(combat.to_string(), EXAMPLE1_FINAL);
        assert_eq!(result, WinnerGoblins(HitPoints(590)));
        assert_eq!(combat.rounds(), 47);
    }

    #[test]
    fn example2_final() {
        let mut combat = parse(EXAMPLE2_INPUT);

        let result = combat.fight();

        assert_eq!(combat.to_string(), EXAMPLE2_FINAL);
        assert_eq!(result, WinnerElves(HitPoints(982)));
        assert_eq!(combat.rounds(), 37);
    }

    #[test]
    fn example3_final() {
        let mut combat = parse(EXAMPLE3_INPUT);

        let result = combat.fight();

        assert_eq!(combat.to_string(), EXAMPLE3_FINAL);
        assert_eq!(result, WinnerElves(HitPoints(859)));
        assert_eq!(combat.rounds(), 46);
    }

    #[test]
    fn example4_final() {
        let mut combat = parse(EXAMPLE4_INPUT);

        let result = combat.fight();

        assert_eq!(combat.to_string(), EXAMPLE4_FINAL);
        assert_eq!(result, WinnerGoblins(HitPoints(793)));
        assert_eq!(combat.rounds(), 35);
    }

    #[test]
    fn example5_final() {
        let mut combat = parse(EXAMPLE5_INPUT);

        let result = combat.fight();

        assert_eq!(combat.to_string(), EXAMPLE5_FINAL);
        assert_eq!(result, WinnerGoblins(HitPoints(536)));
        assert_eq!(combat.rounds(), 54);
    }

    #[test]
    fn example6_final() {
        let mut combat = parse(EXAMPLE6_INPUT);

        let result = combat.fight();

        assert_eq!(combat.to_string(), EXAMPLE6_FINAL);
        assert_eq!(result, WinnerGoblins(HitPoints(937)));
        assert_eq!(combat.rounds(), 20);
    }

    #[ignore]
    #[test]
    fn answer() {
        let combat = parse(INPUT);

        let answer = fight(&combat);

        assert_eq!(answer, 181_952);
    }
}

mod id {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    proptest! {
        #[test]
        fn elf_id_and_goblin_id_of_same_number_have_different_hash(
            id1_val in MIN_ID_VAL..=MAX_ID_VAL,
            id2_val in MIN_ID_VAL..=MAX_ID_VAL,
        ) {
            let id1 = Id::<Elf>::new(id1_val);
            let id2 = Id::<Goblin>::new(id2_val);

            let mut hasher1 = DefaultHasher::default();
            id1.hash(&mut hasher1);

            let mut hasher2 = DefaultHasher::default();
            id2.hash(&mut hasher2);

            prop_assert_ne!(hasher1.finish(), hasher2.finish());
        }
    }
}

mod position {
    use super::*;

    proptest! {
        #[test]
        fn position_at_north_edge_has_no_north_adjacent(
            x in MIN_COORD..=MAX_COORD
        ) {
            let mut position = Position::MIN;
            position.x = x;

            prop_assert_eq!(position.north(), None);
        }
    }

    proptest! {
        #[test]
        fn position_at_south_edge_has_no_south_adjacent(
            x in MIN_COORD..=MAX_COORD
        ) {
            let mut position = Position::MAX;
            position.x = x;

            prop_assert_eq!(position.south(), None);
        }
    }

    proptest! {
        #[test]
        fn position_at_east_edge_has_no_east_adjacent(
            y in MIN_COORD..=MAX_COORD
        ) {
            let mut position = Position::MAX;
            position.y = y;

            prop_assert_eq!(position.east(), None);
        }
    }

    proptest! {
        #[test]
        fn position_at_west_edge_has_no_west_adjacent(
            y in MIN_COORD..=MAX_COORD
        ) {
            let mut position = Position::MIN;
            position.y = y;

            prop_assert_eq!(position.west(), None);
        }
    }

    proptest! {
        #[test]
        fn manhattan_distance_is_commutative(
            x1 in MIN_COORD..=MAX_COORD / 2,
            y1 in MIN_COORD..=MAX_COORD / 2,
            x2 in MIN_COORD..=MAX_COORD / 2,
            y2 in MIN_COORD..=MAX_COORD / 2,
        ) {
            let pos1 = Position { x: x1, y: y1 };
            let pos2 = Position { x: x2, y: y2 };

            prop_assert_eq!(pos1.manhattan_distance(pos2), pos2.manhattan_distance(pos1));
        }
    }
}

mod shortest_path {
    use super::*;

    #[test]
    fn example1_elf1_goblin1() {
        let combat_map = parse(EXAMPLE1_INPUT);

        let path1 = combat_map.shortest_path(Position::new(4, 2), Position::new(2, 1));
        assert_eq!(
            path1,
            vec![
                Position::new(4, 2),
                Position::new(4, 1),
                Position::new(3, 1),
                Position::new(2, 1),
            ]
        );
    }

    #[test]
    fn example1_elf1_goblin2() {
        let combat_map = parse(EXAMPLE1_INPUT);

        let path1 = combat_map.shortest_path(Position::new(4, 2), Position::new(5, 2));
        assert_eq!(path1, vec![Position::new(4, 2), Position::new(5, 2),]);
    }

    #[test]
    fn example1_elf1_goblin3() {
        let combat_map = parse(EXAMPLE1_INPUT);

        let path1 = combat_map.shortest_path(Position::new(4, 2), Position::new(5, 3));
        assert_eq!(path1, vec![]);
    }

    #[test]
    fn example1_elf1_goblin4() {
        let combat_map = parse(EXAMPLE1_INPUT);

        let path1 = combat_map.shortest_path(Position::new(4, 2), Position::new(3, 4));
        assert_eq!(
            path1,
            vec![
                Position::new(4, 2),
                Position::new(3, 2),
                Position::new(3, 3),
                Position::new(3, 4),
            ]
        );
    }
}
