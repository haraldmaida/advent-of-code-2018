use super::*;

const INPUT: &str = include_str!("../../input/2018/day11.txt");

mod calc_cell_power {
    use super::*;

    #[test]
    fn cell_power_example1() {
        let serial_no = SerialNo(8);
        let cell = CellCoord::new(3, 5);

        let cell_power = calc_cell_power(serial_no, cell);

        assert_eq!(cell_power, PowerLevel(4));
    }

    #[test]
    fn cell_power_example2() {
        let serial_no = SerialNo(57);
        let cell = CellCoord::new(122, 79);

        let cell_power = calc_cell_power(serial_no, cell);

        assert_eq!(cell_power, PowerLevel(-5));
    }

    #[test]
    fn cell_power_example3() {
        let serial_no = SerialNo(39);
        let cell = CellCoord::new(217, 196);

        let cell_power = calc_cell_power(serial_no, cell);

        assert_eq!(cell_power, PowerLevel(0));
    }

    #[test]
    fn cell_power_example4() {
        let serial_no = SerialNo(71);
        let cell = CellCoord::new(101, 153);

        let cell_power = calc_cell_power(serial_no, cell);

        assert_eq!(cell_power, PowerLevel(4));
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let serial_no = SerialNo(18);

        let (group, power) = max_power_cell_group(serial_no);

        assert_eq!(group.coord, CellCoord::new(33, 45));
        assert_eq!(power, PowerLevel(29));
    }

    #[test]
    fn example2() {
        let serial_no = SerialNo(42);

        let (group, power) = max_power_cell_group(serial_no);

        assert_eq!(group.coord, CellCoord::new(21, 61));
        assert_eq!(power, PowerLevel(30));
    }

    #[test]
    fn answer() {
        let serial_no = parse(INPUT);

        let answer = best_cell_group(&serial_no);

        assert_eq!(answer, CellCoord::new(20, 83));
    }
}

mod part2 {
    use super::*;

    #[ignore]
    #[test]
    fn example1() {
        let serial_no = SerialNo(18);

        let (group, power) = max_power_cell_group_size(serial_no);

        assert_eq!(group.coord, CellCoord::new(90, 269));
        assert_eq!(group.size, 16);
        assert_eq!(power, PowerLevel(113));
    }

    #[ignore]
    #[test]
    fn example2() {
        let serial_no = SerialNo(42);

        let (group, power) = max_power_cell_group_size(serial_no);

        assert_eq!(group.coord, CellCoord::new(232, 251));
        assert_eq!(group.size, 12);
        assert_eq!(power, PowerLevel(119));
    }

    #[ignore]
    #[test]
    fn answer() {
        let serial_no = parse(INPUT);

        let answer = best_cell_group_size(&serial_no);

        assert_eq!(answer, Answer(237, 281, 10));
    }
}
