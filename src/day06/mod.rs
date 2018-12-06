//! # Day 6:
//!
//! ## Part 1
//!
//! The device on your wrist beeps several times, and once again you feel like
//! you're falling.
//!
//! "Situation critical," the device announces. "Destination indeterminate.
//! Chronal interference detected. Please specify new target coordinates."
//!
//! The device then produces a list of coordinates (your puzzle input). Are they
//! places it thinks are safe or dangerous? It recommends you check manual page
//! 729. The Elves did not give you a manual.
//!
//! If they're dangerous, maybe you can minimize the danger by finding the
//! coordinate that gives the largest distance from the other points.
//!
//! Using only the Manhattan distance, determine the area around each coordinate
//! by counting the number of integer X,Y locations that are closest to that
//! coordinate (and aren't tied in distance to any other coordinate).
//!
//! Your goal is to find the size of the largest area that isn't infinite. For
//! example, consider the following list of coordinates:
//!
//! ```text
//! 1, 1
//! 1, 6
//! 8, 3
//! 3, 4
//! 5, 5
//! 8, 9
//! ```
//!
//! If we name these coordinates A through F, we can draw them on a grid,
//! putting 0,0 at the top left:
//!
//! ```text
//! ..........
//! .A........
//! ..........
//! ........C.
//! ...D......
//! .....E....
//! .B........
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! This view is partial - the actual grid extends infinitely in all directions.
//! Using the Manhattan distance, each location's closest coordinate can be
//! determined, shown here in lowercase:
//!
//! ```text
//! aaaaa.cccc
//! aAaaa.cccc
//! aaaddecccc
//! aadddeccCc
//! ..dDdeeccc
//! bb.deEeecc
//! bBb.eeee..
//! bbb.eeefff
//! bbb.eeffff
//! bbb.ffffFf
//! ```
//!
//! Locations shown as . are equally far from two or more coordinates, and so
//! they don't count as being closest to any.
//!
//! In this example, the areas of coordinates A, B, C, and F are infinite -
//! while not shown here, their areas extend forever outside the visible grid.
//! However, the areas of coordinates D and E are finite: D is closest to 9
//! locations, and E is closest to 17 (both including the coordinate's location
//! itself). Therefore, in this example, the size of the largest area is 17.
//!
//! What is the size of the largest area that isn't infinite?
//!
//! [Advent of Code 2018 - Day 6](https://adventofcode.com/2018/day/6)

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::str::FromStr;

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("can not parse Point in line: {:?}", line))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(coord: (i32, i32)) -> Self {
        Point {
            x: coord.0,
            y: coord.1,
        }
    }
}

impl Point {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Fail, Debug)]
pub enum ParsePointError {
    #[fail(display = "missing x-coordinate in {:?}", _0)]
    MissingXCoordinate(String),
    #[fail(display = "missing y-coordinate in {:?}", _0)]
    MissingYCoordinate(String),
    #[fail(display = "x-coordinate is not an integer, but is {:?}", _0)]
    XNotAnInteger(String),
    #[fail(display = "y-coordinate is not an integer, but is {:?}", _0)]
    YNotAnInteger(String),
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(",");
        let x = coords
            .next()
            .ok_or_else(|| ParsePointError::MissingXCoordinate(s.to_string()))?;
        let y = coords
            .next()
            .ok_or_else(|| ParsePointError::MissingYCoordinate(s.to_string()))?;
        let x = x
            .trim()
            .parse()
            .map_err(|_| ParsePointError::XNotAnInteger(x.to_string()))?;
        let y = y
            .trim()
            .parse()
            .map_err(|_| ParsePointError::YNotAnInteger(y.to_string()))?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance(u32);

impl Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Point {
    /// Manhattan distance from this point to another point.
    fn distance(&self, other: &Point) -> Distance {
        Distance((other.x - self.x).abs() as u32 + (other.y - self.y).abs() as u32)
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Point]) -> u32 {
    let (_point, area) = largest_area(input);
    area
}

pub fn largest_area(points: &[Point]) -> (Point, u32) {
    let min_x = points
        .iter()
        .map(Point::x)
        .min()
        .expect("no point in the list");
    let min_y = points
        .iter()
        .map(Point::y)
        .min()
        .expect("no point in the list");
    let max_x = points
        .iter()
        .map(Point::x)
        .max()
        .expect("no point in the list");
    let max_y = points
        .iter()
        .map(Point::y)
        .max()
        .expect("no point in the list");

    let mut point_map: HashMap<Point, Point> = HashMap::with_capacity(32);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let coord = Point { x, y };
            let mut distance_map: HashMap<Point, Distance> = HashMap::with_capacity(32);
            for point in points {
                distance_map.insert(*point, point.distance(&coord));
            }
            let smallest_distance = distance_map
                .iter()
                .min_by_key(|(_, distance)| *distance)
                .expect("no point in list");
            let nearest_points: Vec<_> = distance_map
                .iter()
                .filter(|(_, distance)| *distance == smallest_distance.1)
                .collect();
            if nearest_points.len() == 1 {
                point_map.insert(coord, *nearest_points[0].0);
            }
        }
    }

    let mut count_map = HashMap::with_capacity(32);
    point_map.into_iter().for_each(|(_, point)| {
        count_map
            .entry(point)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let mut valid_points = HashSet::with_capacity(32);
    for point in points {
        let top_left = points
            .iter()
            .find(|other| other.y < point.y && other.x < point.x);
        let top_right = points
            .iter()
            .find(|other| other.y < point.y && other.x > point.x);
        let bottom_left = points
            .iter()
            .find(|other| other.y > point.y && other.x < point.x);
        let bottom_right = points
            .iter()
            .find(|other| other.y > point.y && other.x > point.x);
        match (top_left, top_right, bottom_left, bottom_right) {
            (Some(_), Some(_), Some(_), Some(_)) => {
                valid_points.insert(point);
            }
            _ => {}
        }
    }

    count_map
        .into_iter()
        .filter(|(point, _)| valid_points.contains(point))
        .max_by_key(|(_, count)| *count)
        .expect("no point in list")
}

//#[aoc(day6, part2)]
//pub fn part2(input: &[Point]) -> u32 {
//    unimplemented!()
//}

#[cfg(test)]
mod tests;
