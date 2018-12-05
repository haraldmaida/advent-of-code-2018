use super::*;

const INPUT: &str = include_str!("../../input/2018/day4.txt");

#[test]
fn parse_falls_asleep() {
    let input = "[1518-05-19 23:51] Guard #577 begins shift\n[1518-05-20 00:03] falls asleep";

    let records = parse(input);

    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0],
        Record {
            guard_id: 577,
            day: "1518-05-20".into(),
            hour: 0,
            minute: 3,
            event: Event::FallAsleep
        }
    )
}

#[test]
fn parse_wakes_up() {
    let input = "[1518-07-18 00:00] Guard #1289 begins shift\n[1518-07-19 00:15] wakes up";

    let records = parse(input);

    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0],
        Record {
            guard_id: 1289,
            day: "1518-07-19".into(),
            hour: 0,
            minute: 15,
            event: Event::WakeUp
        }
    )
}

const EXAMPLE1_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = EXAMPLE1_INPUT;

        let answer = most_asleep_minute(&parse(input));

        assert_eq!(answer, (10, 24));
    }

    #[test]
    fn answer() {
        let answer = strategy1(&parse(INPUT));

        assert_eq!(answer, 14346);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = EXAMPLE1_INPUT;

        let answer = most_frequently_asleep_minute(&parse(input));

        assert_eq!(answer, (99, 45, 3));
    }

    #[test]
    fn answer() {
        let answer = strategy2(&parse(INPUT));

        assert_eq!(answer, 5705);
    }
}
