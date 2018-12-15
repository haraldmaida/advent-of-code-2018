//! # Day 10: The Stars Align
//!
//! It's no use; your navigation system simply isn't capable of providing
//! walking directions in the arctic circle, and certainly not in 1018.
//!
//! The Elves suggest an alternative. In times like these, North Pole rescue
//! operations will arrange points of light in the sky to guide missing Elves
//! back to base. Unfortunately, the message is easy to miss: the points move
//! slowly enough that it takes hours to align them, but have so much momentum
//! that they only stay aligned for a second. If you blink at the wrong time, it
//! might be hours before another message appears.
//!
//! You can see these points of light floating in the distance, and record their
//! position in the sky and their velocity, the relative change in position per
//! second (your puzzle input). The coordinates are all given from your
//! perspective; given enough time, those positions and velocities will move the
//! points into a cohesive message!
//!
//! Rather than wait, you decide to fast-forward the process and calculate what
//! the points will eventually spell.
//!
//! For example, suppose you note the following points:
//!
//! ```text
//! position=< 9,  1> velocity=< 0,  2>
//! position=< 7,  0> velocity=<-1,  0>
//! position=< 3, -2> velocity=<-1,  1>
//! position=< 6, 10> velocity=<-2, -1>
//! position=< 2, -4> velocity=< 2,  2>
//! position=<-6, 10> velocity=< 2, -2>
//! position=< 1,  8> velocity=< 1, -1>
//! position=< 1,  7> velocity=< 1,  0>
//! position=<-3, 11> velocity=< 1, -2>
//! position=< 7,  6> velocity=<-1, -1>
//! position=<-2,  3> velocity=< 1,  0>
//! position=<-4,  3> velocity=< 2,  0>
//! position=<10, -3> velocity=<-1,  1>
//! position=< 5, 11> velocity=< 1, -2>
//! position=< 4,  7> velocity=< 0, -1>
//! position=< 8, -2> velocity=< 0,  1>
//! position=<15,  0> velocity=<-2,  0>
//! position=< 1,  6> velocity=< 1,  0>
//! position=< 8,  9> velocity=< 0, -1>
//! position=< 3,  3> velocity=<-1,  1>
//! position=< 0,  5> velocity=< 0, -1>
//! position=<-2,  2> velocity=< 2,  0>
//! position=< 5, -2> velocity=< 1,  2>
//! position=< 1,  4> velocity=< 2,  1>
//! position=<-2,  7> velocity=< 2, -2>
//! position=< 3,  6> velocity=<-1, -1>
//! position=< 5,  0> velocity=< 1,  0>
//! position=<-6,  0> velocity=< 2,  0>
//! position=< 5,  9> velocity=< 1, -2>
//! position=<14,  7> velocity=<-2,  0>
//! position=<-3,  6> velocity=< 2, -1>
//! ```
//!
//! Each line represents one point. Positions are given as <X, Y> pairs: X
//! represents how far left (negative) or right (positive) the point appears,
//! while Y represents how far up (negative) or down (positive) the point
//! appears.
//!
//! At 0 seconds, each point has the position given. Each second, each point's
//! velocity is added to its position. So, a point with velocity <1, -2> is
//! moving to the right, but is moving upward twice as quickly. If this point's
//! initial position were <3, 9>, after 3 seconds, its position would become
//! <6, 3>.
//!
//! Over time, the points listed above would move like this:
//!
//! Initially:
//! ```text
//! ........#.............
//! ................#.....
//! .........#.#..#.......
//! ......................
//! #..........#.#.......#
//! ...............#......
//! ....#.................
//! ..#.#....#............
//! .......#..............
//! ......#...............
//! ...#...#.#...#........
//! ....#..#..#.........#.
//! .......#..............
//! ...........#..#.......
//! #...........#.........
//! ...#.......#..........
//! ```
//!
//! After 1 second:
//! ```text
//! ......................
//! ......................
//! ..........#....#......
//! ........#.....#.......
//! ..#.........#......#..
//! ......................
//! ......#...............
//! ....##.........#......
//! ......#.#.............
//! .....##.##..#.........
//! ........#.#...........
//! ........#...#.....#...
//! ..#...........#.......
//! ....#.....#.#.........
//! ......................
//! ......................
//! ```
//!
//! After 2 seconds:
//! ```text
//! ......................
//! ......................
//! ......................
//! ..............#.......
//! ....#..#...####..#....
//! ......................
//! ........#....#........
//! ......#.#.............
//! .......#...#..........
//! .......#..#..#.#......
//! ....#....#.#..........
//! .....#...#...##.#.....
//! ........#.............
//! ......................
//! ......................
//! ......................
//! ```
//!
//! After 3 seconds:
//! ```text
//! ......................
//! ......................
//! ......................
//! ......................
//! ......#...#..###......
//! ......#...#...#.......
//! ......#...#...#.......
//! ......#####...#.......
//! ......#...#...#.......
//! ......#...#...#.......
//! ......#...#...#.......
//! ......#...#..###......
//! ......................
//! ......................
//! ......................
//! ......................
//! ```
//!
//! After 4 seconds:
//! ```text
//! ......................
//! ......................
//! ......................
//! ............#.........
//! ........##...#.#......
//! ......#.....#..#......
//! .....#..##.##.#.......
//! .......##.#....#......
//! ...........#....#.....
//! ..............#.......
//! ....#......#...#......
//! .....#.....##.........
//! ...............#......
//! ...............#......
//! ......................
//! ......................
//! ```
//!
//! After 3 seconds, the message appeared briefly: HI. Of course, your message
//! will be much longer and will take many more seconds to appear.
//!
//! What message will eventually appear in the sky?
//!
//! ## Part 2
//!
//! Good thing you didn't have to wait, because that would have taken a long
//! time - much longer than the 3 seconds in the example above.
//!
//! Impressed by your sub-hour communication capabilities, the Elves are
//! curious: exactly how many seconds would they have needed to wait for that
//! message to appear?
//!
//! [Advent of Code 2018 - Day 10](https://adventofcode.com/2018/day/10)

use std::{
    collections::HashSet,
    fmt::{self, Display},
    i32, i64,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration(u64);

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl Duration {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Star {
    pub position: Position,
    pub velocity: Velocity,
}

impl Star {
    fn evolve(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sky {
    stars: Vec<Star>,
}

impl AsRef<Sky> for Sky {
    fn as_ref(&self) -> &Sky {
        self
    }
}

impl Sky {
    pub fn new(stars: Vec<Star>) -> Self {
        Sky { stars }
    }

    pub fn stars(&self) -> &[Star] {
        &self.stars
    }

    pub fn evolve(&mut self) {
        self.stars.iter_mut().for_each(Star::evolve)
    }

    pub fn area(&self) -> (Position, Position) {
        let mut top_left = Position {
            x: i32::MAX,
            y: i32::MAX,
        };
        self.stars.iter().for_each(|star| {
            if star.position.x < top_left.x {
                top_left.x = star.position.x;
            }
            if star.position.y < top_left.y {
                top_left.y = star.position.y;
            }
        });
        let mut bottom_right = Position {
            x: i32::MIN,
            y: i32::MIN,
        };
        self.stars.iter().for_each(|star| {
            if star.position.x > bottom_right.x {
                bottom_right.x = star.position.x;
            }
            if star.position.y > bottom_right.y {
                bottom_right.y = star.position.y;
            }
        });
        (top_left, bottom_right)
    }

    pub fn star_positions(&self) -> HashSet<Position> {
        self.stars.iter().map(|star| star.position).collect()
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\n")?;
        let star_positions = self.star_positions();
        let (top_left, bottom_right) = self.area();
        for y in top_left.y..=bottom_right.y {
            let mut line = String::with_capacity(8);
            for x in top_left.x..=bottom_right.x {
                if star_positions.contains(&Position { x, y }) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            line.push('\n');
            f.write_str(&line)?;
        }
        Ok(())
    }
}

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Sky {
    let stars = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(|c| c == '<' || c == '>' || c == ',');
            let x = parts.nth(1).unwrap().trim().parse::<i32>().unwrap();
            let y = parts.nth(0).unwrap().trim().parse::<i32>().unwrap();
            let vx = parts.nth(1).unwrap().trim().parse::<i32>().unwrap();
            let vy = parts.nth(0).unwrap().trim().parse::<i32>().unwrap();

            let position = Position { x, y };
            let velocity = Velocity { x: vx, y: vy };
            Star { position, velocity }
        })
        .collect();

    Sky::new(stars)
}

#[aoc(day10, part1)]
pub fn align_stars(sky: &Sky) -> Sky {
    let mut last_area = i64::MAX;
    let mut last_sky = sky.clone();
    loop {
        let mut next_sky = last_sky.clone();
        next_sky.evolve();
        let (top_left, bottom_right) = next_sky.area();
        let curr_area =
            i64::from(bottom_right.x - top_left.x) * i64::from(bottom_right.y - top_left.y);
        if curr_area > last_area {
            break;
        } else {
            last_area = curr_area;
            last_sky = next_sky;
        }
    }
    last_sky
}

#[aoc(day10, part2)]
pub fn time_to_aligned_stars(sky: &Sky) -> Duration {
    let mut duration = Duration(0);
    let mut last_area = i64::MAX;
    let mut sky = sky.clone();
    loop {
        sky.evolve();
        let (top_left, bottom_right) = sky.area();
        let curr_area =
            i64::from(bottom_right.x - top_left.x) * i64::from(bottom_right.y - top_left.y);
        if curr_area > last_area {
            break;
        } else {
            last_area = curr_area;
            duration.increment();
        }
    }
    duration
}

#[cfg(test)]
mod tests;
