//! # Advent of Code 2018
//!
//! "We've detected some temporal anomalies," one of Santa's Elves at the
//! Temporal Anomaly Research and Detection Instrument Station tells you. She
//! sounded pretty worried when she called you down here. "At 500-year intervals
//! into the past, someone has been changing Santa's history!"
//!
//! "The good news is that the changes won't propagate to our time stream for
//! another 25 days, and we have a device" - she attaches something to your
//! wrist - "that will let you fix the changes with no such propagation delay.
//! It's configured to send you 500 years further into the past every few days;
//! that was the best we could do on such short notice."
//!
//! "The bad news is that we are detecting roughly fifty anomalies throughout
//! time; the device will indicate fixed anomalies with stars. The other bad
//! news is that we only have one device and you're the best person for the job!
//! Good lu--" She taps a button on the device and you suddenly feel like you're
//! falling. To save Christmas, you need to get all fifty stars by December
//! 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! day in the advent calendar; the second puzzle is unlocked when you complete
//! the first. Each puzzle grants one star. Good luck!
//!
//! [Advent of Code 2018](https://adventofcode.com/2018)

#[cfg(test)]
#[macro_use]
extern crate proptest;

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

aoc_lib! { year = 2018 }
