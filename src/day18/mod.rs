//! # Day 18: Settlers of The North Pole
//!
//! On the outskirts of the North Pole base construction project, many Elves are
//! collecting lumber.
//!
//! The lumber collection area is 50 acres by 50 acres; each acre can be either
//! open ground (.), trees (|), or a lumberyard (#). You take a scan of the area
//! (your puzzle input).
//!
//! Strange magic is at work here: each minute, the landscape looks entirely
//! different. In exactly one minute, an open acre can fill with trees, a wooded
//! acre can be converted to a lumberyard, or a lumberyard can be cleared to
//! open ground (the lumber having been sent to other projects).
//!
//! The change to each acre is based entirely on the contents of that acre as
//! well as the number of open, wooded, or lumberyard acres adjacent to it at
//! the start of each minute. Here, "adjacent" means any of the eight acres
//! surrounding that acre. (Acres on the edges of the lumber collection area
//! might have fewer than eight adjacent acres; the missing acres aren't
//! counted.)
//!
//! In particular:
//!
//! * An open acre will become filled with trees if three or more adjacent acres
//!   contained trees. Otherwise, nothing happens.
//! * An acre filled with trees will become a lumberyard if three or more
//!   adjacent acres were lumberyards. Otherwise, nothing happens.
//! * An acre containing a lumberyard will remain a lumberyard if it was
//!   adjacent to at least one other lumberyard and at least one acre containing
//!   trees. Otherwise, it becomes open.
//!
//! These changes happen across all acres simultaneously, each of them using the
//! state of all acres at the beginning of the minute and changing to their new
//! form by the end of that same minute. Changes that happen during the minute
//! don't affect each other.
//!
//! For example, suppose the lumber collection area is instead only 10 by 10
//! acres with this initial configuration:
//!
//! ```text
//! Initial state:
//! .#.#...|#.
//! .....#|##|
//! .|..|...#.
//! ..|#.....#
//! #.#|||#|#|
//! ...#.||...
//! .|....|...
//! ||...#|.#|
//! |.||||..|.
//! ...#.|..|.
//!
//! After 1 minute:
//! .......##.
//! ......|###
//! .|..|...#.
//! ..|#||...#
//! ..##||.|#|
//! ...#||||..
//! ||...|||..
//! |||||.||.|
//! ||||||||||
//! ....||..|.
//!
//! After 2 minutes:
//! .......#..
//! ......|#..
//! .|.|||....
//! ..##|||..#
//! ..###|||#|
//! ...#|||||.
//! |||||||||.
//! ||||||||||
//! ||||||||||
//! .|||||||||
//!
//! After 3 minutes:
//! .......#..
//! ....|||#..
//! .|.||||...
//! ..###|||.#
//! ...##|||#|
//! .||##|||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 4 minutes:
//! .....|.#..
//! ...||||#..
//! .|.#||||..
//! ..###||||#
//! ...###||#|
//! |||##|||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 5 minutes:
//! ....|||#..
//! ...||||#..
//! .|.##||||.
//! ..####|||#
//! .|.###||#|
//! |||###||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 6 minutes:
//! ...||||#..
//! ...||||#..
//! .|.###|||.
//! ..#.##|||#
//! |||#.##|#|
//! |||###||||
//! ||||#|||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 7 minutes:
//! ...||||#..
//! ..||#|##..
//! .|.####||.
//! ||#..##||#
//! ||##.##|#|
//! |||####|||
//! |||###||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 8 minutes:
//! ..||||##..
//! ..|#####..
//! |||#####|.
//! ||#...##|#
//! ||##..###|
//! ||##.###||
//! |||####|||
//! ||||#|||||
//! ||||||||||
//! ||||||||||
//!
//! After 9 minutes:
//! ..||###...
//! .||#####..
//! ||##...##.
//! ||#....###
//! |##....##|
//! ||##..###|
//! ||######||
//! |||###||||
//! ||||||||||
//! ||||||||||
//!
//! After 10 minutes:
//! .||##.....
//! ||###.....
//! ||##......
//! |##.....##
//! |##.....##
//! |##....##|
//! ||##.####|
//! ||#####|||
//! ||||#|||||
//! ||||||||||
//! ```
//!
//! After 10 minutes, there are 37 wooded acres and 31 lumberyards. Multiplying
//! the number of wooded acres by the number of lumberyards gives the total
//! resource value after ten minutes: 37 * 31 = 1147.
//!
//! What will the total resource value of the lumber collection area be after 10
//! minutes?
//!
//! ## Part 2
//!
//! This important natural resource will need to last for at least thousands of
//! years. Are the Elves collecting this lumber sustainably?
//!
//! What will the total resource value of the lumber collection area be after
//! 1000000000 minutes?
//!
//! [Advent of Code 2018 - Day 18](https://adventofcode.com/2018/day/18)

use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::HashMap,
    fmt::{self, Display},
    iter::FromIterator,
    ops::Add,
};

use self::Resource::*;

pub type Coord = i16;
pub const MAX_COORD: Coord = std::i16::MAX;
pub const MIN_COORD: Coord = std::i16::MIN;

const ADJACENT_OFFSETS: [Position; 8] = [
    Position { x: -1, y: -1 },
    Position { x: 0, y: -1 },
    Position { x: 1, y: -1 },
    Position { x: -1, y: 0 },
    Position { x: 1, y: 0 },
    Position { x: -1, y: 1 },
    Position { x: 0, y: 1 },
    Position { x: 1, y: 1 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: Coord,
    pub y: Coord,
}

impl Position {
    pub const MAX: Position = Position {
        x: MAX_COORD,
        y: MAX_COORD,
    };

    pub const MIN: Position = Position {
        x: MIN_COORD,
        y: MIN_COORD,
    };

    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn adjacent(self) -> Adjacent {
        Adjacent::new(self)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            if self.x == other.x {
                Ordering::Equal
            } else if self.x < other.x {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if self.y < other.y {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<Self> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> <Self as Add<Position>>::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
pub struct Adjacent {
    position: Position,
    offset: usize,
}

impl Adjacent {
    fn new(position: Position) -> Self {
        Self {
            position,
            offset: 0,
        }
    }
}

impl Iterator for Adjacent {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == ADJACENT_OFFSETS.len() {
            return None;
        }
        let position = self.position + ADJACENT_OFFSETS[self.offset];
        self.offset += 1;
        Some(position)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    OpenGround,
    Trees,
    Lumberyard,
}

impl Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            OpenGround => ".",
            Trees => "|",
            Lumberyard => "#",
        };
        f.write_str(symbol)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Area {
    resources: HashMap<Position, Resource>,
}

impl Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line = String::with_capacity(16);
        let (top_left, bottom_right) = self.corners();
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                let symbol = match self.resource(Position::new(x, y)) {
                    OpenGround => '.',
                    Trees => '|',
                    Lumberyard => '#',
                };
                line.push(symbol);
            }
            line.push('\n');
            f.write_str(&line)?;
            line.clear();
        }
        Ok(())
    }
}

impl Area {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            resources: HashMap::with_capacity(capacity),
        }
    }

    pub fn with_resources(resources: impl IntoIterator<Item = (Position, Resource)>) -> Self {
        Self {
            resources: HashMap::from_iter(resources.into_iter()),
        }
    }

    pub fn corners(&self) -> (Position, Position) {
        let mut top_left = Position::MAX;
        let mut bottom_right = Position::MIN;
        for pos in self.resources.keys() {
            if pos.x < top_left.x {
                top_left.x = pos.x;
            }
            if pos.y < top_left.y {
                top_left.y = pos.y;
            }
            if pos.x > bottom_right.x {
                bottom_right.x = pos.x;
            }
            if pos.y > bottom_right.y {
                bottom_right.y = pos.y
            }
        }
        (top_left, bottom_right)
    }

    pub fn resource(&self, position: Position) -> Resource {
        self.resources.get(&position).cloned().unwrap_or(OpenGround)
    }

    pub fn set_resource(&mut self, position: Position, resource: Resource) -> Resource {
        self.resources
            .insert(position, resource)
            .unwrap_or(OpenGround)
    }

    pub fn count_resource(&self, resource: Resource) -> usize {
        self.resources
            .values()
            .filter(|res| **res == resource)
            .count()
    }

    pub fn nth_generation(self, n: usize) -> Area {
        let (top_left, bottom_right) = self.corners();
        if n < 1_000 {
            let mut area = self;
            for no in 1..=n {
                area = area.next_generation(top_left, bottom_right);
                debug!("\n\ngeneration no. {}:\n\n{}", no, area);
            }
            area
        } else {
            let mut break_at_no = n + 1;
            let mut prove = break_at_no;
            let mut first_cycle_start = 1;
            let mut repeat_cycle = 1;
            let mut seen_pattern = HashMap::with_capacity(16);
            let mut area = self;
            seen_pattern.insert(area.to_string(), 0);
            for no in 1..=n {
                area = area.next_generation(top_left, bottom_right);
                let pattern = area.to_string();
                if let Some(prev) = seen_pattern.insert(pattern, no) {
                    let difference = no - prev;
                    if difference != repeat_cycle {
                        repeat_cycle = difference;
                        first_cycle_start = prev;
                        prove = first_cycle_start + repeat_cycle * 4;
                    }
                    debug!(
                        "\n\n{} - {} -- ({}, {}):\n\n{}",
                        no, prev, first_cycle_start, repeat_cycle, area
                    );
                }
                if no == break_at_no {
                    break;
                }
                if no == prove {
                    break_at_no = (n - first_cycle_start) % repeat_cycle + no;
                }
            }
            area
        }
    }

    fn next_generation(&self, top_left: Position, bottom_right: Position) -> Area {
        let mut mutated_area = Area::with_capacity(32);
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                let position = Position::new(x, y);
                match self.resources.get(&position).unwrap_or(&OpenGround) {
                    OpenGround => {
                        let mut num_adj_trees = 0;
                        for adj_pos in position.adjacent() {
                            if Trees == self.resource(adj_pos) {
                                num_adj_trees += 1;
                                if num_adj_trees >= 3 {
                                    mutated_area.resources.insert(position, Trees);
                                    break;
                                }
                            }
                        }
                    },
                    Trees => {
                        let mut num_adj_lumberyards = 0;
                        for adj_pos in position.adjacent() {
                            if Lumberyard == self.resource(adj_pos) {
                                num_adj_lumberyards += 1;
                                if num_adj_lumberyards >= 3 {
                                    break;
                                }
                            }
                        }
                        if num_adj_lumberyards >= 3 {
                            mutated_area.resources.insert(position, Lumberyard);
                        } else {
                            mutated_area.resources.insert(position, Trees);
                        }
                    },
                    Lumberyard => {
                        let mut num_trees = 0;
                        let mut num_lumberyard = 0;
                        for adj_pos in position.adjacent() {
                            match self.resource(adj_pos) {
                                Trees => {
                                    num_trees += 1;
                                },
                                Lumberyard => {
                                    num_lumberyard += 1;
                                },
                                _ => {},
                            }
                            if num_trees >= 1 && num_lumberyard >= 1 {
                                mutated_area.resources.insert(position, Lumberyard);
                                break;
                            }
                        }
                    },
                }
            }
        }
        mutated_area
    }
}

#[aoc_generator(day18)]
pub fn parse(input: &str) -> Area {
    let mut area = Area::new();
    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            let resource = match chr {
                '.' => OpenGround,
                '|' => Trees,
                '#' => Lumberyard,
                _ if chr.is_whitespace() => continue,
                _ => panic!("unexpected character {:?} in line: {:?}", chr, line),
            };
            if resource != OpenGround {
                area.set_resource(Position::new(x as Coord, y as Coord), resource);
            }
        }
    }
    area
}

#[aoc(day18, part1)]
pub fn total_resource_value_after_10_minutes(area: &Area) -> usize {
    let new_area = area.clone().nth_generation(10);
    new_area.count_resource(Trees) * new_area.count_resource(Lumberyard)
}

#[aoc(day18, part2)]
pub fn total_resource_value_after_1_000_000_000_minutes(area: &Area) -> usize {
    let new_area = area.clone().nth_generation(1_000_000_000);
    new_area.count_resource(Trees) * new_area.count_resource(Lumberyard)
}

#[cfg(test)]
mod tests;
