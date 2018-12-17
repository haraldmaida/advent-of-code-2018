//! # Day 4: Repose Record
//!
//! You've sneaked into another supply closet - this time, it's across from the
//! prototype suit manufacturing lab. You need to sneak inside and fix the
//! issues with the suit, but there's a guard stationed outside the lab, so this
//! is as close as you can safely get.
//!
//! As you search the closet for anything that might help, you discover that
//! you're not the first person to want to sneak in. Covering the walls, someone
//! has spent an hour starting every midnight for the past few months secretly
//! observing this guard post! They've been writing down the ID of the one guard
//! on duty that night - the Elves seem to have decided that one guard was
//! enough for the overnight shift - as well as when they fall asleep or wake up
//! while at their post (your puzzle input).
//!
//! For example, consider the following records, which have already been
//! organized into chronological order:
//!
//! ```text
//! [1518-11-01 00:00] Guard #10 begins shift
//! [1518-11-01 00:05] falls asleep
//! [1518-11-01 00:25] wakes up
//! [1518-11-01 00:30] falls asleep
//! [1518-11-01 00:55] wakes up
//! [1518-11-01 23:58] Guard #99 begins shift
//! [1518-11-02 00:40] falls asleep
//! [1518-11-02 00:50] wakes up
//! [1518-11-03 00:05] Guard #10 begins shift
//! [1518-11-03 00:24] falls asleep
//! [1518-11-03 00:29] wakes up
//! [1518-11-04 00:02] Guard #99 begins shift
//! [1518-11-04 00:36] falls asleep
//! [1518-11-04 00:46] wakes up
//! [1518-11-05 00:03] Guard #99 begins shift
//! [1518-11-05 00:45] falls asleep
//! [1518-11-05 00:55] wakes up
//! ```
//!
//! Timestamps are written using year-month-day hour:minute format. The guard
//! falling asleep or waking up is always the one whose shift most recently
//! started. Because all asleep/awake times are during the midnight hour
//! (00:00 - 00:59), only the minute portion (00 - 59) is relevant for those
//! events.
//!
//! Visually, these records show that the guards are asleep at these times:
//!
//! ```text
//! Date   ID   Minute
//!             000000000011111111112222222222333333333344444444445555555555
//!             012345678901234567890123456789012345678901234567890123456789
//! 11-01  #10  .....####################.....#########################.....
//! 11-02  #99  ........................................##########..........
//! 11-03  #10  ........................#####...............................
//! 11-04  #99  ....................................##########..............
//! 11-05  #99  .............................................##########.....
//! ```
//!
//! The columns are Date, which shows the month-day portion of the relevant day;
//! ID, which shows the guard on duty that day; and Minute, which shows the
//! minutes during which the guard was asleep within the midnight hour. (The
//! Minute column's header shows the minute's ten's digit in the first row and
//! the one's digit in the second row.) Awake is shown as ., and asleep is shown
//! as #.
//!
//! Note that guards count as asleep on the minute they fall asleep, and they
//! count as awake on the minute they wake up. For example, because Guard #10
//! wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
//!
//! If you can figure out the guard most likely to be asleep at a specific time,
//! you might be able to trick that guard into working tonight so you can have
//! the best chance of sneaking in. You have two strategies for choosing the
//! best guard/minute combination.
//!
//! **Strategy 1:** Find the guard that has the most minutes asleep. What minute
//! does that guard spend asleep the most?
//!
//! In the example above, Guard #10 spent the most minutes asleep, a total of
//! 50 minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes
//! (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas
//! any other minute the guard was asleep was only seen on one day).
//!
//! While this example listed the entries in chronological order, your entries
//! are in the order you found them. You'll need to organize them before they
//! can be analyzed.
//!
//! What is the ID of the guard you chose multiplied by the minute you chose?
//! <br/>(In the above example, the answer would be 10 * 24 = 240.)
//!
//! ## Part 2
//!
//! **Strategy 2:** Of all guards, which guard is most frequently asleep on the
//! same minute?
//!
//! In the example above, Guard #99 spent minute 45 asleep more than any other
//! guard or minute - three times in total. (In all other cases, any guard spent
//! any minute asleep at most twice.)
//!
//! What is the ID of the guard you chose multiplied by the minute you chose?
//! (In the above example, the answer would be 99 * 45 = 4455.)
//!
//! [Advent of Code 2018 - Day 4](https://adventofcode.com/2018/day/4)

use std::{collections::HashMap, u8};

pub type GuardId = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub guard_id: GuardId,
    pub day: String,
    pub hour: u8,
    pub minute: u8,
    pub event: Event,
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Record> {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();
    let mut records = Vec::with_capacity(16);
    let mut guard_id = 0;
    for line in lines {
        match &line[19..24] {
            "Guard" => {
                guard_id = line[26..]
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap_or_else(|_| {
                        panic!("not a guard id in line {}", line);
                    })
            },
            "falls" => {
                let day = line[1..11].to_owned();
                let hour = line[12..14].parse().unwrap_or_else(|_| {
                    panic!("not an hour in line {}", line);
                });
                let minute = line[15..17].parse().unwrap_or_else(|_| {
                    panic!("not an minute in line {}", line);
                });
                records.push(Record {
                    guard_id,
                    day,
                    hour,
                    minute,
                    event: Event::FallAsleep,
                })
            },
            "wakes" => {
                let day = line[1..11].to_owned();
                let hour = line[12..14]
                    .parse()
                    .unwrap_or_else(|_| panic!("not an hour in line {}", line));
                let minute = line[15..17]
                    .parse()
                    .unwrap_or_else(|_| panic!("not an minute in line {}", line));
                records.push(Record {
                    guard_id,
                    day,
                    hour,
                    minute,
                    event: Event::WakeUp,
                })
            },
            _ => unreachable!(&format!("invalid line: {}", line)),
        }
    }
    records
}

#[aoc(day4, part1)]
pub fn strategy1(input: &[Record]) -> u32 {
    let (guard_id, minute) = most_asleep_minute(input);
    u32::from(guard_id) * minute
}

fn sleeping_periods(input: &[Record]) -> HashMap<GuardId, Vec<(u8, u8)>> {
    let mut sleeping_periods: HashMap<GuardId, Vec<(u8, u8)>> = HashMap::with_capacity(16);
    let mut fall_asleep_time = 0;
    for record in input {
        match record.event {
            Event::FallAsleep => fall_asleep_time = record.minute,
            Event::WakeUp => {
                let sleep_period = (fall_asleep_time, record.minute);
                sleeping_periods
                    .entry(record.guard_id)
                    .and_modify(|times| times.push(sleep_period))
                    .or_insert_with(|| vec![sleep_period]);
            },
        }
    }
    sleeping_periods
}

fn most_asleep_minute(input: &[Record]) -> (GuardId, u32) {
    let sleeping_periods = sleeping_periods(input);

    let most_sleepy_guard = sleeping_periods
        .iter()
        .map(|(guard_id, periods)| {
            (
                guard_id,
                periods
                    .iter()
                    .map(|(from, till)| u32::from(*till - *from))
                    .sum::<u32>(),
            )
        })
        .max_by_key(|(_, total)| *total)
        .map(|(guard_id, _)| guard_id)
        .unwrap_or_else(|| panic!("what? no guard sleeps actually?"));

    let mut sleepy_minutes = HashMap::with_capacity(32);
    for (sleeps_from, sleeps_until) in sleeping_periods[most_sleepy_guard].iter() {
        for minute in *sleeps_from..*sleeps_until {
            sleepy_minutes
                .entry(minute)
                .and_modify(|times| *times += 1)
                .or_insert(1);
        }
    }

    let ovl_minute = sleepy_minutes
        .into_iter()
        .max_by_key(|(_, times)| *times)
        .map(|(minute, _)| minute)
        .expect("it was promised that the input reduces to exactly one minute");

    (*most_sleepy_guard, u32::from(ovl_minute))
}

#[aoc(day4, part2)]
pub fn strategy2(input: &[Record]) -> u32 {
    let (guard_id, minute, _) = most_frequently_asleep_minute(input);
    u32::from(guard_id) * u32::from(minute)
}

fn most_frequently_asleep_minute(input: &[Record]) -> (GuardId, u8, u32) {
    let sleeping_periods = sleeping_periods(input);

    let mut sleepy_minutes: HashMap<GuardId, HashMap<u8, u32>> = HashMap::with_capacity(16);
    sleeping_periods
        .into_iter()
        .for_each(|(guard_id, sleepy_periods)| {
            let guard_minutes = sleepy_minutes
                .entry(guard_id)
                .or_insert_with(|| HashMap::with_capacity(32));
            sleepy_periods.into_iter().for_each(|(from, till)| {
                (from..till).for_each(|minute| {
                    guard_minutes
                        .entry(minute)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                });
            });
        });

    let (guard_id, (minute, count)) = sleepy_minutes
        .into_iter()
        .map(|(guard_id, guard_minutes)| {
            let max_count_minute = guard_minutes
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .unwrap();
            (guard_id, max_count_minute)
        })
        .max_by_key(|(_, (_, count))| *count)
        .unwrap();

    (guard_id, minute, count)
}

#[cfg(test)]
mod tests;
