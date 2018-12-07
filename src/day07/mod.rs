//! # Day 7: The Sum of Its Parts
//!
//! ## Part 1
//!
//! You find yourself standing on a snow-covered coastline; apparently, you
//! landed a little off course. The region is too hilly to see the North Pole
//! from here, but you do spot some Elves that seem to be trying to unpack
//! something that washed ashore. It's quite cold out, so you decide to risk
//! creating a paradox by asking them for directions.
//!
//! "Oh, are you the search party?" Somehow, you can understand whatever Elves
//! from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the
//! device on your wrist also be a translator? "Those clothes don't look very
//! warm; take this." They hand you a heavy coat.
//!
//! "We do need to find our way back to the North Pole, but we have higher
//! priorities at the moment. You see, believe it or not, this box contains
//! something that will solve all of Santa's transportation problems - at least,
//! that's what it looks like from the pictures in the instructions." It doesn't
//! seem like they can read whatever language it's in, but you can: "Sleigh kit.
//! Some assembly required."
//!
//! "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at
//! once!" They start excitedly pulling more parts out of the box.
//!
//! The instructions specify a series of steps and requirements about which
//! steps must be finished before others can begin (your puzzle input). Each
//! step is designated by a single letter. For example, suppose you have the
//! following instructions:
//!
//! ```text
//! Step C must be finished before step A can begin.
//! Step C must be finished before step F can begin.
//! Step A must be finished before step B can begin.
//! Step A must be finished before step D can begin.
//! Step B must be finished before step E can begin.
//! Step D must be finished before step E can begin.
//! Step F must be finished before step E can begin.
//! ```
//!
//! Visually, these requirements look like this:
//!
//! ```text
//!   -->A--->B--
//!  /    \      \
//! C      -->D----->E
//!  \           /
//!   ---->F-----
//! ```
//!
//! Your first goal is to determine the order in which the steps should be
//! completed. If more than one step is ready, choose the step which is first
//! alphabetically. In this example, the steps would be completed as follows:
//!
//! * Only C is available, and so it is done first.
//! * Next, both A and F are available. A is first alphabetically, so it is done
//!   next.
//! * Then, even though F was available earlier, steps B and D are now also
//!   available, and B is the first alphabetically of the three.
//! * After that, only D and F are available. E is not available because only
//!   some of its prerequisites are complete. Therefore, D is completed next.
//! * F is the only choice, so it is done next.
//! * Finally, E is completed.
//!
//! So, in this example, the correct order is CABDFE.
//!
//! In what order should the steps in your instructions be completed?
//!
//! [Advent of Code 2018 - Day 7](https://adventofcode.com/2018/day/7)

use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, IntoIterator};

pub type InstructionId = char;

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    prior_map: HashMap<InstructionId, HashSet<InstructionId>>,
}

impl AsRef<ExecutionPlan> for ExecutionPlan {
    fn as_ref(&self) -> &ExecutionPlan {
        self
    }
}

impl ExecutionPlan {
    pub fn new() -> Self {
        Self {
            prior_map: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            prior_map: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self,
        id: InstructionId,
        prerequisites: impl IntoIterator<Item = InstructionId>,
    ) {
        self.prior_map.insert(id, HashSet::from_iter(prerequisites));
    }

    pub fn add_prerequisite(&mut self, id: InstructionId, prior_id: InstructionId) {
        self.prior_map
            .entry(id)
            .or_insert_with(|| HashSet::with_capacity(1))
            .insert(prior_id);
        self.prior_map
            .entry(prior_id)
            .or_insert_with(|| HashSet::new());
    }

    pub fn prerequisites(&self, instruction_id: &InstructionId) -> &HashSet<InstructionId> {
        &self.prior_map[instruction_id]
    }

    pub fn len(&self) -> usize {
        self.prior_map.len()
    }

    pub fn in_order(&self) -> InOrder {
        InOrder {
            todo: &self.prior_map,
            done: HashSet::with_capacity(self.prior_map.len()),
        }
    }
}

#[derive(Debug)]
pub struct InOrder<'a> {
    todo: &'a HashMap<InstructionId, HashSet<InstructionId>>,
    done: HashSet<InstructionId>,
}

impl<'a> Iterator for InOrder<'a> {
    type Item = InstructionId;

    fn next(&mut self) -> Option<Self::Item> {
        self.todo
            .iter()
            .filter(|(id, prior)| !self.done.contains(id) && self.done.is_superset(prior))
            .map(|(id, _)| *id)
            .min()
            .map(|id| {
                self.done.insert(id);
                id
            })
    }
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> ExecutionPlan {
    let mut instructions = ExecutionPlan::with_capacity(16);
    input.trim().lines().for_each(|line| {
        let chars = line.chars();
        let mut skip5 = chars.skip(5);
        let prior_id = skip5
            .next()
            .expect(&format!("no 6th char in line: {}", line));
        let mut skip30 = skip5.skip(30);
        let id = skip30
            .next()
            .expect(&format!("no 37th char in line: {}", line));
        instructions.add_prerequisite(id, prior_id);
    });
    instructions
}

#[aoc(day7, part1)]
pub fn execution_order(execution_plan: &ExecutionPlan) -> String {
    String::from_iter(execution_plan.in_order())
}

#[cfg(test)]
mod tests;
