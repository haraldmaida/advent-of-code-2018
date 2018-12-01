//! Day 1: Chronal Calibration
//!
//! [Advent of Code - Day 1](https://adventofcode.com/2018/day/1)

use std::collections::HashSet;
use std::str::FromStr;

pub type Frequency = i32;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<Frequency> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(idx, text)| {
            Frequency::from_str(text.trim()).expect(&format!(
                "input text at line {:03} is not an i32 but is {:?}",
                idx + 1,
                text
            ))
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn accumulate(input: &[Frequency]) -> Frequency {
    input.into_iter().sum()
}

#[aoc(day1, part2)]
pub fn calibrate(input: &[Frequency]) -> Frequency {
    let mut history = HashSet::with_capacity(16);
    let mut accumulated = 0;
    history.insert(accumulated);
    'rerun: loop {
        for change in input {
            accumulated += change;
            if !history.insert(accumulated) {
                break 'rerun accumulated;
            }
        }
    }
}

#[cfg(test)]
mod tests;
