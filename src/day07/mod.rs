//! # Day 7: The Sum of Its Parts
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
//! ## Part 2
//!
//! As you're about to begin construction, four of the Elves offer to help.
//! "The sun will set soon; it'll go faster if we work together." Now, you need
//! to account for multiple people working on steps simultaneously. If multiple
//! steps are available, workers should still begin them in alphabetical order.
//!
//! Each step takes 60 seconds plus an amount corresponding to its letter:
//! A=1, B=2, C=3, and so on. So, step A takes 60+1=61 seconds, while step Z
//! takes 60+26=86 seconds. No time is required between steps.
//!
//! To simplify things for the example, however, suppose you only have help from
//! one Elf (a total of two workers) and that each step takes 60 fewer seconds
//! (so that step A takes 1 second and step Z takes 26 seconds). Then, using the
//! same instructions as above, this is how each second would be spent:
//!
//! ```text
//! Second   Worker 1   Worker 2   Done
//!    0        C          .
//!    1        C          .
//!    2        C          .
//!    3        A          F       C
//!    4        B          F       CA
//!    5        B          F       CA
//!    6        D          F       CAB
//!    7        D          F       CAB
//!    8        D          F       CAB
//!    9        D          .       CABF
//!   10        E          .       CABFD
//!   11        E          .       CABFD
//!   12        E          .       CABFD
//!   13        E          .       CABFD
//!   14        E          .       CABFD
//!   15        .          .       CABFDE
//! ```
//!
//! Each row represents one second of time. The Second column identifies how
//! many seconds have passed as of the beginning of that second. Each worker
//! column shows the step that worker is currently doing (or . if they are
//! idle). The Done column shows completed steps.
//!
//! Note that the order of the steps has changed; this is because steps now take
//! time to finish and multiple workers can begin multiple steps simultaneously.
//!
//! In this example, it would take 15 seconds for two workers to complete these
//! steps.
//!
//! With 5 workers and the 60+ second step durations described above, how long
//! will it take to complete all of the steps?
//!
//! [Advent of Code 2018 - Day 7](https://adventofcode.com/2018/day/7)

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    iter::{FromIterator, IntoIterator},
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub type InstructionId = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration(u32);

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl Duration {
    const ZERO: Duration = Duration(0);

    pub fn zero() -> Self {
        Duration(0)
    }

    pub fn from_sec(seconds: u32) -> Self {
        Duration(seconds)
    }

    pub fn secs(self) -> u32 {
        self.0
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Self) -> Self::Output {
        Duration(self.0 + rhs.0)
    }
}

impl Add<u32> for Duration {
    type Output = Duration;

    fn add(self, rhs: u32) -> Self::Output {
        Duration(self.0 + rhs)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl AddAssign<u32> for Duration {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

impl Sub<u32> for Duration {
    type Output = Duration;

    fn sub(self, rhs: u32) -> Self::Output {
        Duration(self.0 - rhs)
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl SubAssign<u32> for Duration {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstructionSet {
    duration_offset: Duration,
}

impl InstructionSet {
    fn new(duration_offset: Duration) -> Self {
        Self { duration_offset }
    }

    fn execution_time(&self, instruction_id: InstructionId) -> Duration {
        self.duration_offset + (instruction_id as u32 - 0x40)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    prior_map: HashMap<InstructionId, HashSet<InstructionId>>,
}

impl AsRef<ExecutionPlan> for ExecutionPlan {
    fn as_ref(&self) -> &ExecutionPlan {
        self
    }
}

impl Default for ExecutionPlan {
    fn default() -> Self {
        Self::new()
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
        self.prior_map.entry(prior_id).or_insert_with(HashSet::new);
    }

    pub fn prerequisites(&self, instruction_id: &InstructionId) -> &HashSet<InstructionId> {
        &self.prior_map[instruction_id]
    }

    pub fn is_empty(&self) -> bool {
        self.prior_map.is_empty()
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

    pub fn simulate(
        &self,
        number_of_workers: u8,
        instruction_set: InstructionSet,
    ) -> ExecutionSimulator {
        let number_of_tasks = self.prior_map.len();
        ExecutionSimulator {
            instruction_set,
            todo: &self.prior_map,
            done: HashSet::with_capacity(number_of_tasks),
            in_progress: HashMap::with_capacity(number_of_workers as usize),
            available_workers: number_of_workers,
        }
    }

    pub fn execution_time(
        &self,
        number_of_workers: u8,
        instruction_set: InstructionSet,
    ) -> Duration {
        let mut execution_time = Duration::zero();

        let simulation = self.simulate(number_of_workers, instruction_set);
        for _progress in simulation {
            execution_time += 1;
        }

        execution_time
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

#[derive(Debug)]
pub struct ExecutionSimulator<'a> {
    instruction_set: InstructionSet,
    todo: &'a HashMap<InstructionId, HashSet<InstructionId>>,
    done: HashSet<InstructionId>,
    in_progress: HashMap<InstructionId, Duration>,
    available_workers: u8,
}

impl<'a> Iterator for ExecutionSimulator<'a> {
    type Item = HashMap<InstructionId, Duration>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut available_tasks: Vec<(InstructionId, Duration)> = Vec::with_capacity(8);
        self.todo
            .iter()
            .filter(|(id, prior)| {
                !self.done.contains(id)
                    && !self.in_progress.contains_key(id)
                    && self.done.is_superset(prior)
            })
            .for_each(|(id, _)| {
                available_tasks.push((*id, self.instruction_set.execution_time(*id)));
            });

        available_tasks.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
        (0..self.available_workers)
            .zip(available_tasks.into_iter())
            .for_each(|(_, (id, duration))| {
                if duration > Duration::ZERO {
                    self.in_progress.insert(id, duration);
                    self.available_workers -= 1;
                }
            });

        if self.in_progress.is_empty() {
            return None;
        }

        let mut finished = HashSet::new();
        for (id, duration) in self.in_progress.iter_mut() {
            *duration -= 1;
            if *duration == Duration::ZERO {
                finished.insert(*id);
            }
        }
        for id in finished.drain() {
            self.in_progress.remove(&id);
            self.available_workers += 1;
            self.done.insert(id);
        }

        Some(self.in_progress.clone())
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
            .unwrap_or_else(|| panic!("no 6th char in line: {}", line));
        let mut skip30 = skip5.skip(30);
        let id = skip30
            .next()
            .unwrap_or_else(|| panic!("no 37th char in line: {}", line));
        instructions.add_prerequisite(id, prior_id);
    });
    instructions
}

#[aoc(day7, part1)]
pub fn execution_order(execution_plan: &ExecutionPlan) -> String {
    String::from_iter(execution_plan.in_order())
}

#[aoc(day7, part2)]
pub fn execution_time(execution_plan: &ExecutionPlan) -> Duration {
    let instruction_set = InstructionSet::new(Duration::from_sec(60));
    execution_plan.execution_time(5, instruction_set)
}

#[cfg(test)]
mod tests;
