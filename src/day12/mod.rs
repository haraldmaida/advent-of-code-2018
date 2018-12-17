//! # Day 12: Subterranean Sustainability
//!
//! The year 518 is significantly more underground than your history books
//! implied. Either that, or you've arrived in a vast cavern network under the
//! North Pole.
//!
//! After exploring a little, you discover a long tunnel that contains a row of
//! small pots as far as you can see to your left and right. A few of them
//! contain plants - someone is trying to grow things in these
//! geothermally-heated caves.
//!
//! The pots are numbered, with 0 in front of you. To the left, the pots are
//! numbered -1, -2, -3, and so on; to the right, 1, 2, 3.... Your puzzle input
//! contains a list of pots from 0 to the right and whether they do (#) or do
//! not (.) currently contain a plant, the initial state. (No other pots
//! currently contain plants.) For example, an initial state of #..##....
//! indicates that pots 0, 3, and 4 currently contain plants.
//!
//! Your puzzle input also contains some notes you find on a nearby table:
//! someone has been trying to figure out how these plants spread to nearby
//! pots. Based on the notes, for each generation of plants, a given pot has or
//! does not have a plant based on whether that pot (and the two pots on either
//! side of it) had a plant in the last generation. These are written as LLCRR
//! => N, where L are pots to the left, C is the current pot being considered, R
//! are the pots to the right, and N is whether the current pot will have a
//! plant in the next generation. For example:
//!
//! * A note like `..#.. => .` means that a pot that contains a plant but with
//!   no plants within two pots of it will not have a plant in it during the
//!   next generation.
//! * A note like `##.## => .` means that an empty pot with two plants on each
//!   side of it will remain empty in the next generation.
//! * A note like `.##.# => #` means that a pot has a plant in a given
//!   generation if, in the previous generation, there were plants in that pot,
//!   the one immediately to the left, and the one two pots to the right, but
//!   not in the ones immediately to the right and two to the left.
//!
//! It's not clear what these plants are for, but you're sure it's important, so
//! you'd like to make sure the current configuration of plants is sustainable
//! by determining what will happen after 20 generations.
//!
//! For example, given the following input:
//!
//! ```text
//! initial state: #..#.#..##......###...###
//!
//! ...## => #
//! ..#.. => #
//! .#... => #
//! .#.#. => #
//! .#.## => #
//! .##.. => #
//! .#### => #
//! #.#.# => #
//! #.### => #
//! ##.#. => #
//! ##.## => #
//! ###.. => #
//! ###.# => #
//! ####. => #
//! ```
//!
//! For brevity, in this example, only the combinations which do produce a plant
//! are listed. (Your input includes all possible combinations.) Then, the next
//! 20 generations will look like this:
//!
//! ```text
//!                  1         2         3
//!        0         0         0         0
//!  0: ...#..#.#..##......###...###...........
//!  1: ...#...#....#.....#..#..#..#...........
//!  2: ...##..##...##....#..#..#..##..........
//!  3: ..#.#...#..#.#....#..#..#...#..........
//!  4: ...#.#..#...#.#...#..#..##..##.........
//!  5: ....#...##...#.#..#..#...#...#.........
//!  6: ....##.#.#....#...#..##..##..##........
//!  7: ...#..###.#...##..#...#...#...#........
//!  8: ...#....##.#.#.#..##..##..##..##.......
//!  9: ...##..#..#####....#...#...#...#.......
//! 10: ..#.#..#...#.##....##..##..##..##......
//! 11: ...#...##...#.#...#.#...#...#...#......
//! 12: ...##.#.#....#.#...#.#..##..##..##.....
//! 13: ..#..###.#....#.#...#....#...#...#.....
//! 14: ..#....##.#....#.#..##...##..##..##....
//! 15: ..##..#..#.#....#....#..#.#...#...#....
//! 16: .#.#..#...#.#...##...#...#.#..##..##...
//! 17: ..#...##...#.#.#.#...##...#....#...#...
//! 18: ..##.#.#....#####.#.#.#...##...##..##..
//! 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
//! 20: .#....##....#####...#######....#.#..##.
//! ```
//!
//! The generation is shown along the left, where 0 is the initial state. The
//! pot numbers are shown along the top, where 0 labels the center pot,
//! negative-numbered pots extend to the left, and positive pots extend toward
//! the right. Remember, the initial state begins at pot 0, which is not the
//! leftmost pot used in this example.
//!
//! After one generation, only seven plants remain. The one in pot 0 matched the
//! rule looking for ..#.., the one in pot 4 matched the rule looking for .#.#.,
//! pot 9 matched .##.., and so on.
//!
//! In this example, after 20 generations, the pots shown as # contain plants,
//! the furthest left of which is pot -2, and the furthest right of which is pot
//! 34. Adding up all the numbers of plant-containing pots after the 20th
//! generation produces 325.
//!
//! After 20 generations, what is the sum of the numbers of all pots which
//! contain a plant?
//!
//! ## Part 2
//!
//! You realize that 20 generations aren't enough. After all, these plants will
//! need to last another 1500 years to even reach your timeline, not to mention
//! your future.
//!
//! After fifty billion (50000000000) generations, what is the sum of the
//! numbers of all pots which contain a plant?
//!
//! [Advent of Code 2018 - Day 12](https://adventofcode.com/2018/day/12)

use std::{
    fmt::{self, Display},
    iter::FromIterator,
};

fn fmt_pot(pot: bool) -> char {
    if pot {
        '#'
    } else {
        '.'
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreedRule {
    pattern: Vec<bool>,
    outcome: bool,
}

impl Display for BreedRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule = String::with_capacity(self.pattern.len() + 5);
        self.pattern.iter().for_each(|&pot| {
            rule.push(fmt_pot(pot));
        });
        rule.push_str(" => ");
        rule.push(fmt_pot(self.outcome));
        f.write_str(&rule)
    }
}

impl BreedRule {
    pub fn new(pattern: Vec<bool>, outcome: bool) -> Self {
        BreedRule { pattern, outcome }
    }

    pub fn pattern(&self) -> &[bool] {
        &self.pattern
    }

    pub fn outcome(&self) -> bool {
        self.outcome
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreedingRules(Vec<BreedRule>);

impl AsRef<[BreedRule]> for BreedingRules {
    fn as_ref(&self) -> &[BreedRule] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PotRow(Vec<bool>);

impl Display for PotRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pots = String::with_capacity(self.0.len());
        self.0.iter().for_each(|&pot| pots.push(fmt_pot(pot)));
        f.write_str(&pots)
    }
}

impl AsRef<[bool]> for PotRow {
    fn as_ref(&self) -> &[bool] {
        &self.0
    }
}

impl AsMut<[bool]> for PotRow {
    fn as_mut(&mut self) -> &mut [bool] {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plantation {
    plants: PotRow,
    breed_rules: BreedingRules,
}

impl AsRef<Plantation> for Plantation {
    fn as_ref(&self) -> &Plantation {
        self
    }
}

impl Plantation {
    pub fn new(
        initial_plants: impl IntoIterator<Item = bool>,
        breeding_rules: impl IntoIterator<Item = BreedRule>,
    ) -> Self {
        Self {
            plants: PotRow(Vec::from_iter(initial_plants.into_iter())),
            breed_rules: BreedingRules(Vec::from_iter(breeding_rules.into_iter())),
        }
    }

    pub fn plants_mut(&mut self) -> &mut [bool] {
        self.plants.as_mut()
    }

    pub fn plants(&self) -> &[bool] {
        self.plants.as_ref()
    }

    pub fn breed_rules(&self) -> &[BreedRule] {
        self.breed_rules.as_ref()
    }
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Plantation {
    let mut lines = input.trim().lines();

    let initial_plants = lines.next().unwrap().trim()[15..]
        .chars()
        .map(|chr| chr == '#');

    let _ = lines.next(); // skip one line

    let breeding_rules = lines.map(|line| {
        let mut parts = line.split("=>");
        let pattern = parts
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|chr| chr == '#')
            .collect();
        let outcome = parts
            .next()
            .unwrap()
            .trim()
            .chars()
            .next()
            .map(|chr| chr == '#')
            .unwrap();
        BreedRule::new(pattern, outcome)
    });

    Plantation::new(initial_plants, breeding_rules)
}

#[aoc(day12, part1)]
pub fn sum_of_pot_numbers_after_20_generations(plantation: &Plantation) -> i64 {
    let (evolved, offset) = evolve_n_generations(plantation, 20);
    sum_of_pot_numbers(&evolved, offset)
}

fn sum_of_pot_numbers(plantation: &Plantation, offset: i64) -> i64 {
    let plants = plantation.plants();
    plants
        .iter()
        .zip(offset..plants.len() as i64 + offset)
        .fold(0, |acc, (&pot, num)| if pot { acc + num } else { acc })
}

fn evolve_n_generations(plantation: &Plantation, num_generations: u64) -> (Plantation, i64) {
    let breed_rules = plantation.breed_rules.clone();
    let pattern_size = breed_rules.0[0].pattern.len();
    let center = pattern_size / 2;
    let outer = &vec![false; pattern_size][..];

    let mut offset = 0;
    let mut plants = plantation.plants.clone();

    while &plants.0[plants.0.len() - pattern_size..] != outer {
        plants.0.push(false);
    }
    while &plants.0[0..pattern_size] != outer {
        plants.0.insert(0, false);
        offset -= 1;
    }

    for gen_num in 0..num_generations {
        let mut next_plants = plants.clone();

        next_plants
            .0
            .iter_mut()
            .skip(center)
            .zip(plants.0.windows(pattern_size))
            .for_each(|(center_pot, pots)| {
                *center_pot = breed_rules
                    .0
                    .iter()
                    .find_map(|rule| {
                        if rule.pattern() == pots {
                            Some(rule.outcome)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);
            });

        let mut delta = 0;
        while &next_plants.0[0..pattern_size] == outer {
            next_plants.0.remove(0);
            delta += 1;
        }
        while &next_plants.0[0..pattern_size] != outer {
            next_plants.0.insert(0, false);
            delta -= 1;
        }
        offset += delta;
        while &next_plants.0[next_plants.0.len() - pattern_size..] != outer {
            next_plants.0.push(false);
        }

        if next_plants == plants {
            offset += (num_generations as i64 - gen_num as i64 - 1) * delta;
            break;
        }
        plants = next_plants;

        debug!("{}", plants);
    }
    (
        Plantation {
            plants,
            breed_rules,
        },
        offset,
    )
}

#[aoc(day12, part2)]
pub fn sum_of_pot_numbers_after_50_000_000_000_generations(plantation: &Plantation) -> i64 {
    let (evolved, offset) = evolve_n_generations(plantation, 50_000_000_000);
    sum_of_pot_numbers(&evolved, offset)
}

#[cfg(test)]
mod tests;
