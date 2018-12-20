//! # Day 17: Reservoir Research
//!
//! You arrive in the year 18. If it weren't for the coat you got in 1018, you
//! would be very cold: the North Pole base hasn't even been constructed.
//!
//! Rather, it hasn't been constructed yet. The Elves are making a little
//! progress, but there's not a lot of liquid water in this climate, so they're
//! getting very dehydrated. Maybe there's more underground?
//!
//! You scan a two-dimensional vertical slice of the ground nearby and discover
//! that it is mostly sand with veins of clay. The scan only provides data with
//! a granularity of square meters, but it should be good enough to determine
//! how much water is trapped there. In the scan, x represents the distance to
//! the right, and y represents the distance down. There is also a spring of
//! water near the surface at x=500, y=0. The scan identifies which square
//! meters are clay (your puzzle input).
//!
//! For example, suppose your scan shows the following veins of clay:
//!
//! ```text
//! x=495, y=2..7
//! y=7, x=495..501
//! x=501, y=3..7
//! x=498, y=2..4
//! x=506, y=1..2
//! x=498, y=10..13
//! x=504, y=10..13
//! y=13, x=498..504
//! ```
//!
//! Rendering clay as #, sand as ., and the water spring as +, and with x
//! increasing to the right and y increasing downward, this becomes:
//!
//! ```text
//!    44444455555555
//!    99999900000000
//!    45678901234567
//!  0 ......+.......
//!  1 ............#.
//!  2 .#..#.......#.
//!  3 .#..#..#......
//!  4 .#..#..#......
//!  5 .#.....#......
//!  6 .#.....#......
//!  7 .#######......
//!  8 ..............
//!  9 ..............
//! 10 ....#.....#...
//! 11 ....#.....#...
//! 12 ....#.....#...
//! 13 ....#######...
//! ```
//!
//! The spring of water will produce water forever. Water can move through sand,
//! but is blocked by clay. Water always moves down when possible, and spreads
//! to the left and right otherwise, filling space that has clay on both sides
//! and falling out otherwise.
//!
//! For example, if five squares of water are created, they will flow downward
//! until they reach the clay and settle there. Water that has come to rest is
//! shown here as ~, while sand through which water has passed (but which is now
//! dry again) is shown as |:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#.|.....#.
//! .#..#.|#......
//! .#..#.|#......
//! .#....|#......
//! .#~~~~~#......
//! .#######......
//! ..............
//! ..............
//! ....#.....#...
//! ....#.....#...
//! ....#.....#...
//! ....#######...
//! ```
//!
//! Two squares of water can't occupy the same location. If another five squares
//! of water are created, they will settle on the first five, filling the clay
//! reservoir a little more:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#.|.....#.
//! .#..#.|#......
//! .#..#.|#......
//! .#~~~~~#......
//! .#~~~~~#......
//! .#######......
//! ..............
//! ..............
//! ....#.....#...
//! ....#.....#...
//! ....#.....#...
//! ....#######...
//! ```
//!
//! Water pressure does not apply in this scenario. If another four squares of
//! water are created, they will stay on the right side of the barrier, and no
//! water will reach the left side:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#.|.....#.
//! .#..#~~#......
//! .#..#~~#......
//! .#~~~~~#......
//! .#~~~~~#......
//! .#######......
//! ..............
//! ..............
//! ....#.....#...
//! ....#.....#...
//! ....#.....#...
//! ....#######...
//! ```
//!
//! At this point, the top reservoir overflows. While water can reach the tiles
//! above the surface of the water, it cannot settle there, and so the next five
//! squares of water settle like this:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#||||...#.
//! .#..#~~#|.....
//! .#..#~~#|.....
//! .#~~~~~#|.....
//! .#~~~~~#|.....
//! .#######|.....
//! ........|.....
//! ........|.....
//! ....#...|.#...
//! ....#...|.#...
//! ....#~~~~~#...
//! ....#######...
//! ```
//!
//! Note especially the leftmost |: the new squares of water can reach this
//! tile, but cannot stop there. Instead, eventually, they all fall to the right
//! and settle in the reservoir below.
//!
//! After 10 more squares of water, the bottom reservoir is also full:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#||||...#.
//! .#..#~~#|.....
//! .#..#~~#|.....
//! .#~~~~~#|.....
//! .#~~~~~#|.....
//! .#######|.....
//! ........|.....
//! ........|.....
//! ....#~~~~~#...
//! ....#~~~~~#...
//! ....#~~~~~#...
//! ....#######...
//! ```
//!
//! Finally, while there is nowhere left for the water to settle, it can reach a
//! few more tiles before overflowing beyond the bottom of the scanned data:
//!
//! ```text
//! ......+.......    (line not counted: above minimum y value)
//! ......|.....#.
//! .#..#||||...#.
//! .#..#~~#|.....
//! .#..#~~#|.....
//! .#~~~~~#|.....
//! .#~~~~~#|.....
//! .#######|.....
//! ........|.....
//! ...|||||||||..
//! ...|#~~~~~#|..
//! ...|#~~~~~#|..
//! ...|#~~~~~#|..
//! ...|#######|..
//! ...|.......|..    (line not counted: below maximum y value)
//! ...|.......|..    (line not counted: below maximum y value)
//! ...|.......|..    (line not counted: below maximum y value)
//! ```
//!
//! How many tiles can be reached by the water? To prevent counting forever,
//! ignore tiles with a y coordinate smaller than the smallest y coordinate in
//! your scan data or larger than the largest one. Any x coordinate is valid. In
//! this example, the lowest y coordinate given is 1, and the highest is 13,
//! causing the water spring (in row 0) and the water falling off the bottom of
//! the render (in rows 14 through infinity) to be ignored.
//!
//! So, in the example above, counting both water at rest (~) and other sand
//! tiles the water can hypothetically reach (|), the total number of tiles the
//! water can reach is 57.
//!
//! How many tiles can the water reach within the range of y values in your
//! scan?
//!
//! [Advent of Code 2018 - Day 17](https://adventofcode.com/2018/day/17)

use std::{
    collections::HashMap,
    fmt::{self, Display},
    iter::FromIterator,
};

use self::Matter::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Matter {
    Sand,
    Clay,
    Water,
}

impl Display for Matter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Sand => ".",
            Clay => "#",
            Water => "O",
        };
        f.write_str(symbol)
    }
}

pub type Coord = i16;

pub const MAX_COORD: Coord = std::i16::MAX;
pub const MIN_COORD: Coord = std::i16::MIN;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: Coord,
    pub y: Coord,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
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
}

pub fn area_of_tiles(tiles: &HashMap<Position, Matter>) -> (Position, Position) {
    let mut top_left = Position::MAX;
    let mut bottom_right = Position::MIN;
    for position in tiles.keys() {
        if position.x < top_left.x {
            top_left.x = position.x
        }
        if position.y < top_left.y {
            top_left.y = position.y
        }
        if position.x > bottom_right.x {
            bottom_right.x = position.x
        }
        if position.y > bottom_right.y {
            bottom_right.y = position.y
        }
    }
    (top_left, bottom_right)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spring(pub Position);

impl Default for Spring {
    fn default() -> Self {
        Spring(Position::new(500, 0))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scan {
    tiles: HashMap<Position, Matter>,
}

impl Display for Scan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_tiles(&self.tiles, f)
    }
}

impl Scan {
    pub fn new(tiles: impl IntoIterator<Item = (Position, Matter)>) -> Self {
        Self {
            tiles: HashMap::from_iter(tiles.into_iter()),
        }
    }

    pub fn area(&self) -> (Position, Position) {
        area_of_tiles(&self.tiles)
    }

    pub fn walk_water_course(self, spring: Spring, max_y: Coord) -> WaterCourse {
        WaterCourse::new(self.tiles, spring, max_y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaterCourse {
    spring: Spring,
    max_y: Coord,
    tiles: HashMap<Position, Matter>,
    to_check: Vec<(Position, bool)>,
}

impl WaterCourse {
    fn new(tiles: HashMap<Position, Matter>, spring: Spring, max_y: Coord) -> Self {
        let mut to_check = Vec::with_capacity(8);
        to_check.push((Position::new(spring.0.x, spring.0.y + 1), true));
        Self {
            spring,
            max_y,
            tiles,
            to_check,
        }
    }

    pub fn area(&self) -> (Position, Position) {
        area_of_tiles(&self.tiles)
    }
}

impl Iterator for WaterCourse {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("{:?}", self.to_check);
        while let Some((c_pos, is_spring)) = self.to_check.pop() {
            let below = Position::new(c_pos.x, c_pos.y + 1);
            if self.tiles.contains_key(&below) {
                if is_spring && c_pos.y > 0 {
                    let upper = Position::new(c_pos.x, c_pos.y - 1);
                    self.to_check.push((upper, true));
                }
                let left = Position::new(c_pos.x - 1, c_pos.y);
                let left_matter = self.tiles.get(&left);
                let right = Position::new(c_pos.x + 1, c_pos.y);
                let right_matter = self.tiles.get(&right);
                match (left_matter, right_matter) {
                    (None, None)
                    | (None, Some(Sand))
                    | (Some(Sand), None)
                    | (Some(Sand), Some(Sand)) => {
                        if is_spring {
                            self.to_check.pop();
                            self.to_check.push((c_pos, true));
                        }
                        self.to_check.push((right, false));
                        self.to_check.push((left, false));
                    },
                    (None, Some(_)) | (Some(Sand), Some(_)) => {
                        self.to_check.push((left, false));
                    },
                    (Some(_), None) | (Some(_), Some(Sand)) => {
                        self.to_check.push((right, false));
                    },
                    (Some(_), Some(_)) => {},
                }
                if self.tiles.insert(c_pos, Water) != None {
                    continue;
                }
                return Some(c_pos);
            } else {
                if let Some((_, true)) = self.to_check.last() {
                    self.to_check.pop();
                }
                if below.y <= self.max_y {
                    self.to_check.push((below, true));
                }
                self.tiles.insert(c_pos, Water);
                return Some(c_pos);
            }
        }
        None
    }
}

impl Display for WaterCourse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_tiles(&self.tiles, f)
    }
}

fn format_tiles(tiles: &HashMap<Position, Matter>, f: &mut fmt::Formatter) -> fmt::Result {
    let (top_left, bottom_right) = area_of_tiles(tiles);
    let width = (bottom_right.x - top_left.x) as usize;
    for y in 0..=bottom_right.y {
        let mut line = String::with_capacity(width);
        for x in top_left.x - 1..=bottom_right.x + 1 {
            let position = Position::new(x, y);
            let symbol = if let Some(matter) = tiles.get(&position) {
                match matter {
                    Sand => '.',
                    Clay => '#',
                    Water => 'O',
                }
            } else {
                '.'
            };
            line.push(symbol);
        }
        line.push('\n');
        f.write_str(&line)?;
    }
    Ok(())
}

#[aoc_generator(day17)]
pub fn parse(input: &str) -> Scan {
    let mut clay_tiles = HashMap::with_capacity(32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Value {
        Xmin,
        Xmax,
        Ymin,
        Ymax,
    }

    for line in input.lines() {
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        let mut param = Value::Xmin;
        let mut value = String::with_capacity(3);

        let mut save_value = |param: Value, value: &str| {
            let number = value.parse::<Coord>().unwrap();
            match param {
                Value::Xmin => {
                    x_min = number;
                    x_max = number;
                },
                Value::Xmax => x_max = number,
                Value::Ymin => {
                    y_min = number;
                    y_max = number;
                },
                Value::Ymax => y_max = number,
            }
        };

        for chr in line.chars() {
            match chr {
                'x' => param = Value::Xmin,
                'y' => param = Value::Ymin,
                '.' => {
                    if !value.is_empty() {
                        save_value(param, &value);
                        value.clear();
                    }
                    param = match param {
                        Value::Xmin => Value::Xmax,
                        Value::Ymin => Value::Ymax,
                        _ => param,
                    }
                },
                ',' => {
                    if !value.is_empty() {
                        save_value(param, &value);
                        value.clear();
                    }
                },
                '=' => {},
                _ if chr.is_digit(10) => value.push(chr),
                _ if chr.is_whitespace() => {},
                _ => panic!(format!("unexpected character {} in line {}", chr, line)),
            }
        }
        if !value.is_empty() {
            save_value(param, &value);
            value.clear();

            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    clay_tiles.insert(Position::new(x, y), Matter::Clay);
                }
            }
        }
    }

    Scan::new(clay_tiles)
}

#[aoc(day17, part1)]
pub fn num_tiles_flooded_by_water(scan: &Scan) -> usize {
    let (top_left, bottom_right) = scan.area();
    let mut spring = Spring::default();
    spring.0.y = top_left.y - 1;
    let mut water_walker = scan.clone().walk_water_course(spring, bottom_right.y);
    let mut num_water_tiles = 0;
    while let Some(_) = water_walker.next() {
        debug!("{}", water_walker);
        num_water_tiles += 1;
    }
    debug!("{}", water_walker);
    num_water_tiles
}

#[cfg(test)]
mod tests;
