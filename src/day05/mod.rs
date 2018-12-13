//! # Day 5: Alchemical Reduction
//!
//! You've managed to sneak in to the prototype suit manufacturing lab. The
//! Elves are making decent progress, but are still struggling with the suit's
//! size reduction capabilities.
//!
//! While the very latest in 1518 alchemical technology might have solved their
//! problem eventually, you can do better. You scan the chemical composition of
//! the suit's material and discover that it is formed by extremely long
//! polymers (one of which is available as your puzzle input).
//!
//! The polymer is formed by smaller units which, when triggered, react with
//! each other such that two adjacent units of the same type and opposite
//! polarity are destroyed. Units' types are represented by letters; units'
//! polarity is represented by capitalization. For instance, r and R are units
//! with the same type but opposite polarity, whereas r and s are entirely
//! different types and do not react.
//!
//! For example:
//!
//! * In aA, a and A react, leaving nothing behind.
//! * In abBA, bB destroys itself, leaving aA. As above, this then destroys
//!   itself, leaving nothing.
//! * In abAB, no two adjacent units are of the same type, and so nothing
//!   happens.
//! * In aabAAB, even though aa and AA are of the same type, their polarities
//!   match, and so nothing happens.
//!
//! Now, consider a larger example, dabAcCaCBAcCcaDA:
//!
//! ```text
//! dabAcCaCBAcCcaDA  The first 'cC' is removed.
//! dabAaCBAcCcaDA    This creates 'Aa', which is removed.
//! dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
//! dabCBAcaDA        No further actions can be taken.
//! ```
//!
//! After all possible reactions, the resulting polymer contains 10 units.
//!
//! How many units remain after fully reacting the polymer you scanned?
//!
//! ## Part 2
//!
//! Time to improve the polymer.
//!
//! One of the unit types is causing problems; it's preventing the polymer from
//! collapsing as much as it should. Your goal is to figure out which unit type
//! is causing the most problems, remove all instances of it (regardless of
//! polarity), fully react the remaining polymer, and measure its length.
//!
//! For example, again using the polymer dabAcCaCBAcCcaDA from above:
//!
//! * Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer
//!   produces dbCBcD, which has length 6.
//! * Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this
//!   polymer produces daCAcaDA, which has length 8.
//! * Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer
//!   produces daDA, which has length 4.
//! * Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this
//!   polymer produces abCBAc, which has length 6.
//!
//! In this example, removing all C/c units was best, producing the answer 4.
//!
//! What is the length of the shortest polymer you can produce by removing all
//! units of exactly one type and fully reacting the result?
//!
//! [Advent of Code 2018 - Day 5](https://adventofcode.com/2018/day/5)

use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

#[aoc(day5, part1)]
pub fn reduced_polymer_len(input: &str) -> usize {
    reduce_polymer(input.trim()).len()
}

fn reduce_polymer(input: &str) -> String {
    let mut polymer = input.to_owned();
    let mut polymer_length = polymer.len();
    let mut skip_next = false;
    loop {
        let mut reduced_polymer = String::with_capacity(polymer_length);
        polymer
            .chars()
            .zip(polymer.chars().skip(1))
            .for_each(|(char1, char2)| {
                if skip_next {
                    skip_next = false;
                } else if char1 != char2 && char1.to_ascii_lowercase() == char2.to_ascii_lowercase()
                {
                    skip_next = true;
                } else {
                    reduced_polymer.push(char1);
                }
            });

        let mut reduced_length = reduced_polymer.len();
        if skip_next {
            skip_next = false;
        } else if ((polymer_length - reduced_length) & 1) == 1 {
            reduced_polymer.push(polymer.chars().last().unwrap());
            reduced_length += 1;
        }
        if reduced_length == polymer_length {
            return reduced_polymer;
        }
        polymer_length = reduced_length;
        polymer = reduced_polymer;
    }
}

#[aoc(day5, part2)]
pub fn improved_polymer_len(input: &str) -> usize {
    let (_, best_polymer) = improve_polymer(input.trim());
    best_polymer.len()
}

fn improve_polymer(input: &str) -> (char, String) {
    let mut unit_types = HashSet::with_capacity(26);
    input.chars().for_each(|c| {
        unit_types.insert(c.to_ascii_lowercase());
    });

    let mut improved_polymers = HashMap::with_capacity(unit_types.len());
    for unit_type in unit_types {
        let polymer = String::from_iter(
            input
                .chars()
                .filter(|c| unit_type != c.to_ascii_lowercase()),
        );
        let reduced_polymer = reduce_polymer(&polymer);
        improved_polymers.insert(unit_type, reduced_polymer);
    }

    improved_polymers
        .into_iter()
        .min_by_key(|(_, polymer)| polymer.len())
        .expect("no best polymer found")
}

#[cfg(test)]
mod tests;
