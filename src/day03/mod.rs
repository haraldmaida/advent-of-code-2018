//! # Day 3: No Matter How You Slice It
//!
//! The Elves managed to locate the chimney-squeeze prototype fabric for Santa's
//! suit (thanks to someone who helpfully wrote its box IDs on the wall of the
//! warehouse in the middle of the night). Unfortunately, anomalies are still
//! affecting them - nobody can even agree on how to cut the fabric.
//!
//! The whole piece of fabric they're working on is a very large square - at
//! least 1000 inches on each side.
//!
//! Each Elf has made a claim about which area of fabric would be ideal for
//! Santa's suit. All claims have an ID and consist of a single rectangle with
//! edges parallel to the edges of the fabric. Each claim's rectangle is defined
//! as follows:
//!
//! * The number of inches between the left edge of the fabric and the left edge
//!   of the rectangle.
//! * The number of inches between the top edge of the fabric and the top edge
//!   of the rectangle.
//! * The width of the rectangle in inches.
//! * The height of the rectangle in inches.
//!
//! A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle
//! 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and
//! 4 inches tall. Visually, it claims the square inches of fabric represented
//! by # (and ignores the square inches of fabric represented by .) in the
//! diagram below:
//!
//! ```text
//! ...........
//! ...........
//! ...#####...
//! ...#####...
//! ...#####...
//! ...#####...
//! ...........
//! ...........
//! ...........
//! ```
//!
//! The problem is that many of the claims overlap, causing two or more claims
//! to cover part of the same areas. For example, consider the following claims:
//!
//! ```text
//! #1 @ 1,3: 4x4
//! #2 @ 3,1: 4x4
//! #3 @ 5,5: 2x2
//! ```
//!
//! Visually, these claim the following areas:
//!
//! ```text
//! ........
//! ...2222.
//! ...2222.
//! .11XX22.
//! .11XX22.
//! .111133.
//! .111133.
//! ........
//! ```
//!
//! The four square inches marked with X are claimed by both 1 and 2. (Claim 3,
//! while adjacent to the others, does not overlap either of them.)
//!
//! If the Elves all proceed with their own plans, none of them will have enough
//! fabric. How many square inches of fabric are within two or more claims?
//!
//! ## Part 2
//!
//! Amidst the chaos, you notice that exactly one claim doesn't overlap by even
//! a single square inch of fabric with any other claim. If you can somehow draw
//! attention to it, maybe the Elves will be able to make Santa's suit after
//! all!
//!
//! For example, in the claims above, only claim 3 is intact after all claims
//! are made.
//!
//! What is the ID of the only claim that doesn't overlap?
//!
//! [Advent of Code 2018 - Day 3](https://adventofcode.com/2018/day/3)

use std::{collections::HashSet, iter::FromIterator, num::ParseIntError, str::FromStr};

pub type ClaimId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim {
    id: ClaimId,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    Id,
    Left,
    Top,
    Width,
    Height,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnknownToken(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        ParseError::ParseIntError(value)
    }
}

impl FromStr for Claim {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.chars();

        let mut token = Token::Unknown;
        let mut id = String::new();
        let mut left = String::new();
        let mut top = String::new();
        let mut width = String::new();
        let mut height = String::new();

        for char in input {
            match char {
                '#' => token = Token::Id,
                '@' => token = Token::Left,
                ',' => token = Token::Top,
                ':' => token = Token::Width,
                'x' => token = Token::Height,
                _ if char.is_whitespace() => {},
                _ if char.is_digit(10) => match token {
                    Token::Id => id.push(char),
                    Token::Left => left.push(char),
                    Token::Top => top.push(char),
                    Token::Width => width.push(char),
                    Token::Height => height.push(char),
                    Token::Unknown => {
                        return Err(ParseError::UnknownToken(format!(
                            "unexpected character {} while parsing {:?}",
                            char, token
                        )));
                    },
                },
                _ => {
                    return Err(ParseError::UnknownToken(format!(
                        "unexpected character {} in {}",
                        char, s
                    )));
                },
            }
        }

        let id = id.parse()?;
        let left = left.parse()?;
        let top = top.parse()?;
        let width = width.parse()?;
        let height = height.parse()?;

        Ok(Claim {
            id,
            left,
            top,
            width,
            height,
        })
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Claim> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| match Claim::from_str(line) {
            Ok(claim) => claim,
            Err(err) => panic!("error at line {}: {:?}", idx + 1, err),
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn overlapping_area(input: &[Claim]) -> usize {
    let mut overlapping_pieces = HashSet::with_capacity(32);

    for (idx, claim1) in input.iter().enumerate() {
        let a1 = claim1.left;
        let b1 = claim1.left + claim1.width;
        let c1 = claim1.top;
        let d1 = claim1.top + claim1.height;

        for claim2 in input.iter().skip(idx + 1) {
            let a2 = claim2.left;
            let b2 = claim2.left + claim2.width;
            let c2 = claim2.top;
            let d2 = claim2.top + claim2.height;

            if let Some((x1, x2, y1, y2)) = if a2 >= a1 && a2 <= b1 {
                if c2 >= c1 && c2 <= d1 {
                    Some((a2, b1.min(b2), c2, d1.min(d2)))
                } else if c2 <= c1 && d2 >= c1 {
                    Some((a2, b1.min(b2), c1, d1.min(d2)))
                } else {
                    None
                }
            } else if a2 <= a1 && b2 >= a1 {
                if c2 >= c1 && c2 <= d1 {
                    Some((a1, b1.min(b2), c2, d1.min(d2)))
                } else if c2 <= c1 && d2 >= c1 {
                    Some((a1, b1.min(b2), c1, d1.min(d2)))
                } else {
                    None
                }
            } else {
                None
            } {
                for y in y1..y2 {
                    for x in x1..x2 {
                        overlapping_pieces.insert((x, y));
                    }
                }
            }
        }
    }

    overlapping_pieces.len()
}

#[aoc(day3, part2)]
pub fn non_overlapping_claims(input: &[Claim]) -> ClaimId {
    let mut non_overlapping_claims: HashSet<ClaimId> =
        HashSet::from_iter(input.iter().map(|claim| claim.id));

    for (idx, claim1) in input.iter().enumerate() {
        let a1 = claim1.left;
        let b1 = claim1.left + claim1.width - 1;
        let c1 = claim1.top;
        let d1 = claim1.top + claim1.height - 1;

        for claim2 in input.iter().skip(idx + 1) {
            let a2 = claim2.left;
            let b2 = claim2.left + claim2.width - 1;
            let c2 = claim2.top;
            let d2 = claim2.top + claim2.height - 1;

            if ((a2 >= a1 && a2 <= b1) && ((c2 >= c1 && c2 <= d1) || (c2 <= c1 && d2 >= c1)))
                || ((a2 <= a1 && b2 >= a1) && ((c2 >= c1 && c2 <= d1) || (c2 <= c1 && d2 >= c1)))
            {
                non_overlapping_claims.remove(&claim1.id);
                non_overlapping_claims.remove(&claim2.id);
            }
        }
    }

    non_overlapping_claims
        .into_iter()
        .next()
        .expect("no non overlapping claim found")
}

#[cfg(test)]
mod tests;
