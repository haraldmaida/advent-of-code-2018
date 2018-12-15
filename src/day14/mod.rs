//! # Day 14: Chocolate Charts
//!
//! You finally have a chance to look at all of the produce moving around.
//! Chocolate, cinnamon, mint, chili peppers, nutmeg, vanilla... the Elves must
//! be growing these plants to make hot chocolate! As you realize this, you hear
//! a conversation in the distance. When you go to investigate, you discover two
//! Elves in what appears to be a makeshift underground kitchen/laboratory.
//!
//! The Elves are trying to come up with the ultimate hot chocolate recipe;
//! they're even maintaining a scoreboard which tracks the quality score (0-9)
//! of each recipe.
//!
//! Only two recipes are on the board: the first recipe got a score of 3, the
//! second, 7. Each of the two Elves has a current recipe: the first Elf starts
//! with the first recipe, and the second Elf starts with the second recipe.
//!
//! To create new recipes, the two Elves combine their current recipes. This
//! creates new recipes from the digits of the sum of the current recipes'
//! scores. With the current recipes' scores of 3 and 7, their sum is 10, and so
//! two new recipes would be created: the first with score 1 and the second with
//! score 0. If the current recipes' scores were 2 and 3, the sum, 5, would only
//! create one recipe (with a score of 5) with its single digit.
//!
//! The new recipes are added to the end of the scoreboard in the order they are
//! created. So, after the first round, the scoreboard is 3, 7, 1, 0.
//!
//! After all new recipes are added to the scoreboard, each Elf picks a new
//! current recipe. To do this, the Elf steps forward through the scoreboard a
//! number of recipes equal to 1 plus the score of their current recipe. So,
//! after the first round, the first Elf moves forward 1 + 3 = 4 times, while
//! the second Elf moves forward 1 + 7 = 8 times. If they run out of recipes,
//! they loop back around to the beginning. After the first round, both Elves
//! happen to loop around until they land on the same recipe that they had in
//! the beginning; in general, they will move to different recipes.
//!
//! Drawing the first Elf as parentheses and the second Elf as square brackets,
//! they continue this process:
//!
//! ```text
//! (3)[7]
//! (3)[7] 1  0
//!  3  7  1 [0](1) 0
//!  3  7  1  0 [1] 0 (1)
//! (3) 7  1  0  1  0 [1] 2
//!  3  7  1  0 (1) 0  1  2 [4]
//!  3  7  1 [0] 1  0 (1) 2  4  5
//!  3  7  1  0 [1] 0  1  2 (4) 5  1
//!  3 (7) 1  0  1  0 [1] 2  4  5  1  5
//!  3  7  1  0  1  0  1  2 [4](5) 1  5  8
//!  3 (7) 1  0  1  0  1  2  4  5  1  5  8 [9]
//!  3  7  1  0  1  0  1 [2] 4 (5) 1  5  8  9  1  6
//!  3  7  1  0  1  0  1  2  4  5 [1] 5  8  9  1 (6) 7
//!  3  7  1  0 (1) 0  1  2  4  5  1  5 [8] 9  1  6  7  7
//!  3  7 [1] 0  1  0 (1) 2  4  5  1  5  8  9  1  6  7  7  9
//!  3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2
//! ```
//!
//! The Elves think their skill will improve after making a few recipes (your
//! puzzle input). However, that could take ages; you can speed this up
//! considerably by identifying the scores of the ten recipes after that. For
//! example:
//!
//! * If the Elves think their skill will improve after making 9 recipes, the
//!   scores of the ten recipes after the first nine on the scoreboard would be
//!   5158916779 (highlighted in the last line of the diagram).
//! * After 5 recipes, the scores of the next ten would be 0124515891.
//! * After 18 recipes, the scores of the next ten would be 9251071085.
//! * After 2018 recipes, the scores of the next ten would be 5941429882.
//!
//! What are the scores of the ten recipes immediately after the number of
//! recipes in your puzzle input?
//!
//! ## Part 2
//!
//! As it turns out, you got the Elves' plan backwards. They actually want to
//! know how many recipes appear on the scoreboard to the left of the first
//! recipes whose scores are the digits from your puzzle input.
//!
//! * 51589 first appears after 9 recipes.
//! * 01245 first appears after 5 recipes.
//! * 92510 first appears after 18 recipes.
//! * 59414 first appears after 2018 recipes.
//!
//! How many recipes appear on the scoreboard to the left of the score sequence
//! in your puzzle input?
//!
//! [Advent of Code 2018 - Day 14](https://adventofcode.com/2018/day/14)

use std::{
    fmt::{self, Display},
    iter::FromIterator,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreSeq(Vec<u8>);

impl Display for ScoreSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted = String::with_capacity(21);
        formatted.push('[');
        for score in &self.0 {
            formatted.push(char::from(*score + 0x30));
            formatted.push(',');
            formatted.push(' ');
        }
        formatted.pop();
        formatted.pop();
        formatted.push(']');
        f.write_str(&formatted)
    }
}

impl From<&[u8]> for ScoreSeq {
    fn from(value: &[u8]) -> Self {
        ScoreSeq(Vec::from_iter(value.iter().cloned()))
    }
}

impl AsRef<[u8]> for ScoreSeq {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromIterator<u8> for ScoreSeq {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        ScoreSeq(Vec::from_iter(iter.into_iter()))
    }
}

impl FromStr for ScoreSeq {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());
        for chr in s.chars() {
            let digit = chr
                .to_digit(10)
                .ok_or_else(|| format!("not a number: {}", s))?;
            digits.push(digit as u8);
        }
        Ok(ScoreSeq(digits))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scoreboard(Vec<u8>);

impl Display for Scoreboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted = String::with_capacity(self.0.len() * 3);
        for score in &self.0 {
            formatted.push(char::from(score + 0x30));
            formatted.push(' ');
            formatted.push(' ');
        }
        formatted.pop();
        formatted.pop();
        f.write_str(&formatted)
    }
}

impl FromIterator<u8> for Scoreboard {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Scoreboard(Vec::from_iter(iter.into_iter()))
    }
}

impl Scoreboard {
    pub fn new(scores: impl IntoIterator<Item = u8>) -> Self {
        Scoreboard(Vec::from_iter(scores.into_iter()))
    }

    pub fn score_seq(&self, num_recipes: usize, num_scores: usize) -> ScoreSeq {
        ScoreSeq::from(&self.0[num_recipes..num_recipes + num_scores])
    }
}

#[derive(Debug)]
pub struct Recipes {
    sequence: Vec<u8>,
    elf1: usize,
    elf2: usize,
    current: usize,
}

impl Recipes {
    pub fn new(recipe1: u8, recipe2: u8) -> Self {
        Recipes {
            sequence: vec![recipe1, recipe2],
            elf1: 0,
            elf2: 1,
            current: 0,
        }
    }
}

impl<'a> Iterator for Recipes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current >= self.sequence.len() {
            let sum = self.sequence[self.elf1] + self.sequence[self.elf2];
            if sum >= 10 {
                self.sequence.push(1);
            }
            self.sequence.push(sum % 10);
            let seq_len = self.sequence.len();
            self.elf1 = (self.elf1 + self.sequence[self.elf1] as usize + 1) % seq_len;
            self.elf2 = (self.elf2 + self.sequence[self.elf2] as usize + 1) % seq_len;
        }
        let next = self.sequence[self.current];
        self.current += 1;
        Some(next)
    }
}

#[aoc(day14, part1)]
pub fn score_seq_after_num_recipes(input: &str) -> ScoreSeq {
    let num_recipes = input
        .trim()
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("not a valid integer: {:?}", input.trim()));
    let num_scores = 10;
    let recipes = Recipes::new(3, 7);
    let scoreboard = Scoreboard::from_iter(recipes.take(num_recipes + num_scores));
    scoreboard.score_seq(num_recipes, num_scores)
}

#[aoc(day14, part2)]
pub fn num_needed_recipes(input: &str) -> usize {
    let score_seq = ScoreSeq::from_str(input.trim())
        .unwrap_or_else(|_| panic!("not a valid integer: {:?}", input.trim()));
    let score_seq_len = score_seq.as_ref().len();
    let mut recipes = Recipes::new(3, 7);
    loop {
        let seq_len = recipes.sequence.len();
        if seq_len > score_seq_len {
            let offset = seq_len - score_seq_len;
            if &recipes.sequence[offset..] == score_seq.as_ref() {
                break offset;
            }
            if &recipes.sequence[offset - 1..seq_len - 1] == score_seq.as_ref() {
                break offset - 1;
            }
        }
        recipes.next();
    }
}

#[cfg(test)]
mod tests;
