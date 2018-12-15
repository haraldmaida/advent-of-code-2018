//! # Day 6:
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
//! ## Part 2
//!
//! On the other hand, if the coordinates are safe, maybe the best you can do is
//! try to find a region near as many coordinates as possible.
//!
//! For example, suppose you want the sum of the Manhattan distance to all of
//! the coordinates to be less than 32. For each location, add up the distances
//! to all of the given coordinates; if the total of those distances is less
//! than 32, that location is within the desired region. Using the same
//! coordinates as above, the resulting region looks like this:
//!
//! ```text
//! ..........
//! .A........
//! ..........
//! ...###..C.
//! ..#D###...
//! ..###E#...
//! .B.###....
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! In particular, consider the highlighted location 4,3 located at the top
//! middle of the region. Its calculation is as follows, where abs() is the
//! absolute value function:
//!
//! * Distance to coordinate A: abs(4-1) + abs(3-1) =  5
//! * Distance to coordinate B: abs(4-1) + abs(3-6) =  6
//! * Distance to coordinate C: abs(4-8) + abs(3-3) =  4
//! * Distance to coordinate D: abs(4-3) + abs(3-4) =  2
//! * Distance to coordinate E: abs(4-5) + abs(3-5) =  3
//! * Distance to coordinate F: abs(4-8) + abs(3-9) = 10
//! * Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
//!
//! Because the total distance to all coordinates (30) is less than 32, the
//! location is within the region.
//!
//! This region, which also includes coordinates D and E, has a total size of
//! 16.
//!
//! Your actual region will need to be much larger than this example, though,
//! instead including all locations with a total distance of less than 10000.
//!
//! What is the size of the region containing all locations which have a total
//! distance to all given coordinates of less than 10000?
//!
//! [Advent of Code 2018 - Day 6](https://adventofcode.com/2018/day/6)

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    iter::Sum,
    str::FromStr,
};

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|_| panic!("can not parse Point in line: {:?}", line))
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
    pub fn x(self) -> i32 {
        self.x
    }

    pub fn y(self) -> i32 {
        self.y
    }

    /// Manhattan distance from this point to another point.
    fn distance(self, other: Point) -> Distance {
        Distance((other.x - self.x).abs() as u32 + (other.y - self.y).abs() as u32)
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
        let mut coords = s.split(',');
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
pub struct Distance(pub u32);

impl Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sum for Distance {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        Distance(iter.fold(0, |acc, a| acc + a.0))
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Point]) -> u32 {
    let (_point, area) = largest_area(input);
    area
}

pub fn largest_area(points: &[Point]) -> (Point, u32) {
    let (top_left, bottom_right) = viewport(points);

    let mut point_map: HashMap<Point, Point> = HashMap::with_capacity(32);
    for x in top_left.x..=bottom_right.x {
        for y in top_left.y..=bottom_right.y {
            let coord = Point { x, y };
            let mut distance_map: HashMap<Point, Distance> = HashMap::with_capacity(32);
            for point in points {
                distance_map.insert(*point, point.distance(coord));
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
        #[cfg_attr(feature = "cargo-clippy", allow(clippy::single_match))]
        match (top_left, top_right, bottom_left, bottom_right) {
            (Some(_), Some(_), Some(_), Some(_)) => {
                valid_points.insert(point);
            },
            _ => {},
        }
    }

    count_map
        .into_iter()
        .filter(|(point, _)| valid_points.contains(point))
        .max_by_key(|(_, count)| *count)
        .expect("no point in list")
}

fn viewport(points: &[Point]) -> (Point, Point) {
    let min_x = points
        .iter()
        .map(|point| point.x)
        .min()
        .expect("no point in the list");
    let min_y = points
        .iter()
        .map(|point| point.y)
        .min()
        .expect("no point in the list");
    let max_x = points
        .iter()
        .map(|point| point.x)
        .max()
        .expect("no point in the list");
    let max_y = points
        .iter()
        .map(|point| point.y)
        .max()
        .expect("no point in the list");
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Point]) -> u32 {
    area_within_distance(Distance(10_000), input)
}

pub fn area_within_distance(target_distance: Distance, points: &[Point]) -> u32 {
    points_within_distance(target_distance, points).len() as u32
}

fn points_within_distance(target_distance: Distance, points: &[Point]) -> HashSet<Point> {
    let mut distance_map: HashMap<Point, Distance> = HashMap::with_capacity(32);
    let (top_left, bottom_right) = viewport(points);
    for x in top_left.x..=bottom_right.x {
        for y in top_left.y..=bottom_right.y {
            let coord = Point { x, y };
            let total_distance = points.iter().map(|point| point.distance(coord)).sum();
            if total_distance < target_distance {
                distance_map.insert(coord, total_distance);
            }
        }
    }
    distance_map.keys().map(ToOwned::to_owned).collect()
}

#[cfg(test)]
mod tests;
