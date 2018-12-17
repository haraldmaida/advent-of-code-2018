//! # Day 11: Chronal Charge
//!
//! You watch the Elves and their sleigh fade into the distance as they head
//! toward the North Pole.
//!
//! Actually, you're the one fading. The falling sensation returns.
//!
//! The low fuel warning light is illuminated on your wrist-mounted device.
//! Tapping it once causes it to project a hologram of the situation: a 300x300
//! grid of fuel cells and their current power levels, some negative. You're not
//! sure what negative power means in the context of time travel, but it can't
//! be good.
//!
//! Each fuel cell has a coordinate ranging from 1 to 300 in both the X
//! (horizontal) and Y (vertical) direction. In X,Y notation, the top-left cell
//! is 1,1, and the top-right cell is 300,1.
//!
//! The interface lets you select any 3x3 square of fuel cells. To increase your
//! chances of getting to your destination, you decide to choose the 3x3 square
//! with the largest total power.
//!
//! The power level in a given fuel cell can be found through the following
//! process:
//!
//! * Find the fuel cell's rack ID, which is its X coordinate plus 10.
//! * Begin with a power level of the rack ID times the Y coordinate.
//! * Increase the power level by the value of the grid serial number (your
//!   puzzle input).
//! * Set the power level to itself multiplied by the rack ID.
//! * Keep only the hundreds digit of the power level (so 12345 becomes 3;
//!   numbers with no hundreds digit become 0).
//! * Subtract 5 from the power level.
//!
//! For example, to find the power level of the fuel cell at 3,5 in a grid with
//! serial number 8:
//!
//! * The rack ID is 3 + 10 = 13.
//! * The power level starts at 13 * 5 = 65.
//! * Adding the serial number produces 65 + 8 = 73.
//! * Multiplying by the rack ID produces 73 * 13 = 949.
//! * The hundreds digit of 949 is 9.
//! * Subtracting 5 produces 9 - 5 = 4.
//!
//! So, the power level of this fuel cell is 4.
//!
//! Here are some more example power levels:
//!
//! * Fuel cell at  122,79, grid serial number 57: power level -5.
//! * Fuel cell at 217,196, grid serial number 39: power level  0.
//! * Fuel cell at 101,153, grid serial number 71: power level  4.
//!
//! Your goal is to find the 3x3 square which has the largest total power. The
//! square must be entirely within the 300x300 grid. Identify this square using
//! the X,Y coordinate of its top-left fuel cell. For example:
//!
//! For grid serial number 18, the largest total 3x3 square has a top-left
//! corner of 33,45 (with a total power of 29); these fuel cells appear in the
//! middle of this 5x5 region:
//!
//! ```text
//! -2  -4   4   4   4
//! -4   4   4   4  -5
//!  4   3   3   4  -4
//!  1   1   2   4  -3
//! -1   0   2  -5  -2
//! ```
//!
//! For grid serial number 42, the largest 3x3 square's top-left is 21,61 (with
//! a total power of 30); they are in the middle of this region:
//!
//! ```text
//! -3   4   2   2   2
//! -4   4   3   3   4
//! -5   3   3   4  -4
//!  4   3   3   4  -3
//!  3   3   3  -5  -1
//! ```
//!
//! What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with
//! the largest total power?
//!
//! ## Part 2
//!
//! You discover a dial on the side of the device; it seems to let you select a
//! square of any size, not just 3x3. Sizes from 1x1 to 300x300 are supported.
//!
//! Realizing this, you now must find the square of any size with the largest
//! total power. Identify this square by including its size as a third parameter
//! after the top-left coordinate: a 9x9 square with a top-left corner of 3,5 is
//! identified as 3,5,9.
//!
//! For example:
//!
//! * For grid serial number 18, the largest total square (with a total power of
//!   113) is 16x16 and has a top-left corner of 90,269, so its identifier is
//!   90,269,16.
//! * For grid serial number 42, the largest total square (with a total power of
//!   119) is 12x12 and has a top-left corner of 232,251, so its identifier is
//!   232,251,12.
//!
//! What is the X,Y,size identifier of the square with the largest total power?
//!
//! [Advent of Code 2018 - Day 11](https://adventofcode.com/2018/day/11)

use std::{
    fmt::{self, Display},
    iter::Sum,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerialNo(pub u32);

impl AsRef<SerialNo> for SerialNo {
    fn as_ref(&self) -> &SerialNo {
        self
    }
}

impl FromStr for SerialNo {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(SerialNo)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PowerLevel(pub i32);

impl Display for PowerLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} W", self.0)
    }
}

impl From<i32> for PowerLevel {
    fn from(val: i32) -> Self {
        PowerLevel(val)
    }
}

impl Sum for PowerLevel {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        PowerLevel(iter.map(|pwl| pwl.0).sum())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub x: u32,
    pub y: u32,
}

impl Display for CellCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}

impl CellCoord {
    pub const MIN: CellCoord = CellCoord { x: 1, y: 1 };

    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RackId(pub u32);

impl From<CellCoord> for RackId {
    fn from(val: CellCoord) -> Self {
        RackId(val.x + 10)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerGrid {
    serial_no: SerialNo,
    size: u32,
}

impl PowerGrid {
    pub fn new(serial_no: SerialNo, size: u32) -> Self {
        Self { serial_no, size }
    }

    pub fn serial_no(&self) -> SerialNo {
        self.serial_no
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn cell_groups(&self, group_size: u32) -> Groups {
        Groups::new(self, group_size)
    }
}

fn calc_cell_power(serial_no: SerialNo, cell: CellCoord) -> PowerLevel {
    let rack_id = RackId::from(cell).0 as i32;
    PowerLevel((rack_id * cell.y as i32 + serial_no.0 as i32) * rack_id % 1000 / 100 - 5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellGroup {
    size: u32,
    coord: CellCoord,
}

impl CellGroup {
    pub fn new(size: u32, coord: CellCoord) -> Self {
        Self { size, coord }
    }

    pub fn coord(&self) -> CellCoord {
        self.coord
    }

    pub fn cells(&self) -> Cells {
        Cells::new(self)
    }

    pub fn power_level(&self, grid: &PowerGrid) -> PowerLevel {
        self.cells()
            .map(|cell| calc_cell_power(grid.serial_no, cell))
            .sum()
    }
}

#[derive(Debug)]
pub struct Groups {
    group_size: u32,
    max: u32,
    current: CellCoord,
}

impl Groups {
    fn new(grid: &PowerGrid, group_size: u32) -> Self {
        Self {
            group_size,
            max: grid.size - group_size,
            current: CellCoord { x: 0, y: 1 },
        }
    }
}

impl Iterator for Groups {
    type Item = CellGroup;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x <= self.max {
            self.current.x += 1;
            Some(CellGroup::new(self.group_size, self.current))
        } else if self.current.y <= self.max {
            self.current.y += 1;
            self.current.x = 1;
            Some(CellGroup::new(self.group_size, self.current))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Cells {
    x0: u32,
    max: CellCoord,
    current: CellCoord,
}

impl Cells {
    fn new(group: &CellGroup) -> Self {
        Self {
            x0: group.coord.x,
            max: CellCoord {
                x: group.coord.x + group.size - 1,
                y: group.coord.y + group.size - 1,
            },
            current: CellCoord {
                x: group.coord.x - 1,
                y: group.coord.y,
            },
        }
    }
}

impl Iterator for Cells {
    type Item = CellCoord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x < self.max.x {
            self.current.x += 1;
            Some(self.current)
        } else if self.current.y < self.max.y {
            self.current.y += 1;
            self.current.x = self.x0;
            Some(self.current)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Answer(u32, u32, u32);

impl Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

impl From<(CellGroup, PowerLevel)> for Answer {
    fn from((group, _): (CellGroup, PowerLevel)) -> Self {
        Answer(group.coord.x, group.coord.y, group.size)
    }
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> SerialNo {
    input.trim().parse().unwrap()
}

#[aoc(day11, part1)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
pub fn best_cell_group(serial_no: &SerialNo) -> CellCoord {
    let (group, _power) = max_power_cell_group(*serial_no);
    group.coord()
}

fn max_power_cell_group(serial_no: SerialNo) -> (CellGroup, PowerLevel) {
    let power_grid = PowerGrid::new(serial_no, 300);
    power_grid
        .cell_groups(3)
        .map(|group| {
            let power = group.power_level(&power_grid);
            debug!("{:?}: {}", group, power);
            (group, power)
        })
        .max_by_key(|(_, level)| *level)
        .unwrap()
}

#[aoc(day11, part2)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
pub fn best_cell_group_size(serial_no: &SerialNo) -> Answer {
    max_power_cell_group_size(*serial_no).into()
}

fn max_power_cell_group_size(serial_no: SerialNo) -> (CellGroup, PowerLevel) {
    let power_grid = PowerGrid::new(serial_no, 300);
    let mut max_per_group_size = Vec::with_capacity(power_grid.size() as usize);
    (1..=power_grid.size())
        .map(|group_size| {
            let (group, power) = power_grid
                .cell_groups(group_size)
                .map(|group| (group, group.power_level(&power_grid)))
                .max_by_key(|(_, level)| *level)
                .unwrap();
            debug!("{:?} = {}", group, power);
            (group, power)
        })
        .for_each(|result| max_per_group_size.push(result));
    max_per_group_size
        .into_iter()
        .max_by_key(|(_, level)| *level)
        .unwrap()
}

#[cfg(test)]
mod tests;
