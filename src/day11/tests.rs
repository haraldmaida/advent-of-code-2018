use super::*;

const INPUT: &str = include_str!("../../input/2018/day11.txt");

mod power_grid {
    use super::*;

    #[test]
    fn cell_power_example1() {
        let power_grid = PowerGrid::new(SerialNo(8), 300);
        let cell = CellCoord::new(3, 5);

        let cell_power = power_grid.cell_power(cell);

        assert_eq!(cell_power, PowerLevel(4));
    }

    #[test]
    fn cell_power_example2() {
        let power_grid = PowerGrid::new(SerialNo(57), 300);
        let cell = CellCoord::new(122, 79);

        let cell_power = power_grid.cell_power(cell);

        assert_eq!(cell_power, PowerLevel(-5));
    }

    #[test]
    fn cell_power_example3() {
        let power_grid = PowerGrid::new(SerialNo(39), 300);
        let cell = CellCoord::new(217, 196);

        let cell_power = power_grid.cell_power(cell);

        assert_eq!(cell_power, PowerLevel(0));
    }

    #[test]
    fn cell_power_example4() {
        let power_grid = PowerGrid::new(SerialNo(71), 300);
        let cell = CellCoord::new(101, 153);

        let cell_power = power_grid.cell_power(cell);

        assert_eq!(cell_power, PowerLevel(4));
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let serial_no = SerialNo(18);

        let (group, power) = max_power_cell_group(&serial_no);

        assert_eq!(group.coord, CellCoord::new(33, 45));
        assert_eq!(power, PowerLevel(29));
    }

    #[test]
    fn example2() {
        let serial_no = SerialNo(42);

        let (group, power) = max_power_cell_group(&serial_no);

        assert_eq!(group.coord, CellCoord::new(21, 61));
        assert_eq!(power, PowerLevel(30));
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let answer = best_cell_group(&input);

        assert_eq!(answer, CellCoord::new(20, 83));
    }
}
