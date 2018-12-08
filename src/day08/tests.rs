use super::*;

const INPUT: &str = include_str!("../../input/2018/day8.txt");

const EXAMPLE1_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

mod parse_input {
    use super::*;

    #[test]
    fn example1() {
        let license = parse(EXAMPLE1_INPUT);

        assert_eq!(
            license,
            License::new(
                vec![
                    Node::new(0, 1),
                    Node::new(1, 2),
                    Node::new(1, 3),
                    Node::new(3, 4),
                ],
                vec![
                    (NodeId(1), vec![Metadata(1), Metadata(1), Metadata(2)]),
                    (NodeId(2), vec![Metadata(10), Metadata(11), Metadata(12)]),
                    (NodeId(3), vec![Metadata(2)]),
                    (NodeId(4), vec![Metadata(99)]),
                ]
            )
        );
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let license = parse(EXAMPLE1_INPUT);

        let answer = license.metadata_checksum();

        assert_eq!(answer, 138);
    }

    #[test]
    fn answer() {
        let license = parse(INPUT);

        let answer = metadata_checksum(&license);

        assert_eq!(answer, 40746);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let license = parse(EXAMPLE1_INPUT);

        let answer = license.value(ROOT.id);

        assert_eq!(answer, 66);
    }

    #[test]
    fn answer() {
        let license = parse(INPUT);

        let answer = value_of_license_root(&license);

        assert_eq!(answer, 37453);
    }
}

mod license {
    use super::*;

    #[test]
    fn child_nodes_of_root() {
        let license = parse(EXAMPLE1_INPUT);

        let child_nodes = license.child_nodes(ROOT.id);

        assert!(child_nodes.contains(&Node::new(1, 2)));
        assert!(child_nodes.contains(&Node::new(1, 3)));
    }

    #[test]
    fn child_nodes_of_node1() {
        let license = parse(EXAMPLE1_INPUT);

        let child_nodes = license.child_nodes(NodeId(2));

        assert!(child_nodes.is_empty());
    }
}
