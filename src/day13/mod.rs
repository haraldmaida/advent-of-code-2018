//! # Day 13: Mine Cart Madness
//!
//! A crop of this size requires significant logistics to transport produce,
//! soil, fertilizer, and so on. The Elves are very busy pushing things around
//! in carts on some kind of rudimentary system of tracks they've come up with.
//!
//! Seeing as how cart-and-track systems don't appear in recorded history for
//! another 1000 years, the Elves seem to be making this up as they go along.
//! They haven't even figured out how to avoid collisions yet.
//!
//! You map out the tracks (your puzzle input) and see where you can help.
//!
//! Tracks consist of straight paths (| and -), curves (/ and \), and
//! intersections (+). Curves connect exactly two perpendicular pieces of track;
//! for example, this is a closed loop:
//!
//! ```text
//! /----\
//! |    |
//! |    |
//! \----/
//! ```
//!
//! Intersections occur when two perpendicular paths cross. At an intersection,
//! a cart is capable of turning left, turning right, or continuing straight.
//! Here are two loops connected by two intersections:
//!
//! ```text
//! /-----\
//! |     |
//! |  /--+--\
//! |  |  |  |
//! \--+--/  |
//!    |     |
//!    \-----/
//! ```
//!
//! Several carts are also on the tracks. Carts always face either up (^), down
//! (v), left (<), or right (>). (On your initial map, the track under each cart
//! is a straight path matching the direction the cart is facing.)
//!
//! Each time a cart has the option to turn (by arriving at any intersection),
//! it turns left the first time, goes straight the second time, turns right the
//! third time, and then repeats those directions starting again with left the
//! fourth time, straight the fifth time, and so on. This process is independent
//! of the particular intersection at which the cart has arrived - that is, the
//! cart has no per-intersection memory.
//!
//! Carts all move at the same speed; they take turns moving a single step at a
//! time. They do this based on their current location: carts on the top row
//! move first (acting from left to right), then carts on the second row move
//! (again from left to right), then carts on the third row, and so on. Once
//! each cart has moved one step, the process repeats; each of these loops is
//! called a tick.
//!
//! For example, suppose there are two carts on a straight track:
//!
//! ```text
//! |  |  |  |  |
//! v  |  |  |  |
//! |  v  v  |  |
//! |  |  |  v  X
//! |  |  ^  ^  |
//! ^  ^  |  |  |
//! |  |  |  |  |
//! ```
//!
//! First, the top cart moves. It is facing down (v), so it moves down one
//! square. Second, the bottom cart moves. It is facing up (^), so it moves up
//! one square. Because all carts have moved, the first tick ends. Then, the
//! process repeats, starting with the first cart. The first cart moves down,
//! then the second cart moves up - right into the first cart, colliding with
//! it! (The location of the crash is marked with an X.) This ends the second
//! and last tick.
//!
//! Here is a longer example:
//!
//! ```text
//! /->-\
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  | v  |
//! \-+-/  \-+--/
//!   \------/
//!
//! /-->\
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \->--/
//!   \------/
//!
//! /---v
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \-+>-/
//!   \------/
//!
//! /---\
//! |   v  /----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \-+->/
//!   \------/
//!
//! /---\
//! |   |  /----\
//! | /->--+-\  |
//! | | |  | |  |
//! \-+-/  \-+--^
//!   \------/
//!
//! /---\
//! |   |  /----\
//! | /-+>-+-\  |
//! | | |  | |  ^
//! \-+-/  \-+--/
//!   \------/
//!
//! /---\
//! |   |  /----\
//! | /-+->+-\  ^
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/
//!
//! /---\
//! |   |  /----<
//! | /-+-->-\  |
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/
//!
//! /---\
//! |   |  /---<\
//! | /-+--+>\  |
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/
//!
//! /---\
//! |   |  /--<-\
//! | /-+--+-v  |
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/
//!
//! /---\
//! |   |  /-<--\
//! | /-+--+-\  |
//! | | |  | v  |
//! \-+-/  \-+--/
//!   \------/
//!
//! /---\
//! |   |  /<---\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \-<--/
//!   \------/
//!
//! /---\
//! |   |  v----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \<+--/
//!   \------/
//!
//! /---\
//! |   |  /----\
//! | /-+--v-\  |
//! | | |  | |  |
//! \-+-/  ^-+--/
//!   \------/
//!
//! /---\
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  X |  |
//! \-+-/  \-+--/
//!   \------/
//! ```
//!
//! After following their respective paths for a while, the carts eventually
//! crash. To help prevent crashes, you'd like to know the location of the first
//! crash. Locations are given in X,Y coordinates, where the furthest left
//! column is X=0 and the furthest top row is Y=0:
//!
//! ```text
//!            111
//!  0123456789012
//! 0/---\
//! 1|   |  /----\
//! 2| /-+--+-\  |
//! 3| | |  X |  |
//! 4\-+-/  \-+--/
//! 5  \------/
//! ```
//!
//! In this example, the location of the first crash is 7,3.
//!
//! ## Part 2
//!
//! There isn't much you can do to prevent crashes in this ridiculous system.
//! However, by predicting the crashes, the Elves know where to be in advance
//! and instantly remove the two crashing carts the moment any crash occurs.
//!
//! They can proceed like this for a while, but eventually, they're going to run
//! out of carts. It could be useful to figure out where the last cart that
//! hasn't crashed will end up.
//!
//! For example:
//!
//! ```text
//! />-<\
//! |   |
//! | /<+-\
//! | | | v
//! \>+</ |
//!   |   ^
//!   \<->/
//!
//! /---\
//! |   |
//! | v-+-\
//! | | | |
//! \-+-/ |
//!   |   |
//!   ^---^
//!
//! /---\
//! |   |
//! | /-+-\
//! | v | |
//! \-+-/ |
//!   ^   ^
//!   \---/
//!
//! /---\
//! |   |
//! | /-+-\
//! | | | |
//! \-+-/ ^
//!   |   |
//!   \---/
//! ```
//!
//! After four very expensive crashes, a tick ends with only one cart remaining;
//! its final location is 6,4.
//!
//! What is the location of the last cart at the end of the first tick where it
//! is the only cart left?
//!
//! [Advent of Code 2018 - Day 13](https://adventofcode.com/2018/day/13)

use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Index, IndexMut},
    u32,
};

use self::MoveResult::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn x(self) -> u32 {
        self.x
    }

    pub fn y(self) -> u32 {
        self.y
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Direction::*;
        let symbol = match *self {
            North => "\u{2191}",
            East => "\u{2192}",
            South => "\u{2193}",
            West => "\u{2190}",
        };
        f.write_str(symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtIntersection {
    TurnLeft,
    GoStraight,
    TurnRight,
}

impl Default for AtIntersection {
    fn default() -> Self {
        AtIntersection::TurnLeft
    }
}

impl AtIntersection {
    pub fn next(self) -> Self {
        use self::AtIntersection::*;
        match self {
            TurnLeft => GoStraight,
            GoStraight => TurnRight,
            TurnRight => TurnLeft,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Moved,
    Collision(Cart, Cart),
    IllegalMove(Direction, RailKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CartNo(u8);

impl Display for CartNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CartNo {
    pub fn new(value: u8) -> Self {
        CartNo(value)
    }

    pub fn val(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cart {
    pub number: CartNo,
    pub position: Position,
    pub direction: Direction,
    pub at_intersection: AtIntersection,
}

impl Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {}", self.number, self.position, self.direction)
    }
}

impl Cart {
    pub fn new(number: CartNo, position: Position, direction: Direction) -> Self {
        Self {
            number,
            position,
            direction,
            at_intersection: AtIntersection::default(),
        }
    }

    pub fn move_one_step(&mut self, rail: RailKind) -> MoveResult {
        use self::AtIntersection::*;
        use self::Direction::*;
        use self::RailKind::*;

        match (self.direction, rail) {
            (North, NorthSouth) => {
                self.position.y -= 1;
            },
            (South, NorthSouth) => {
                self.position.y += 1;
            },
            (West, WestEast) => {
                self.position.x -= 1;
            },
            (East, WestEast) => {
                self.position.x += 1;
            },
            (North, LeftTurn) => {
                self.direction = West;
                self.position.x -= 1;
            },
            (South, LeftTurn) => {
                self.direction = East;
                self.position.x += 1;
            },
            (West, LeftTurn) => {
                self.direction = North;
                self.position.y -= 1;
            },
            (East, LeftTurn) => {
                self.direction = South;
                self.position.y += 1;
            },
            (North, RightTurn) => {
                self.direction = East;
                self.position.x += 1;
            },
            (South, RightTurn) => {
                self.direction = West;
                self.position.x -= 1;
            },
            (West, RightTurn) => {
                self.direction = South;
                self.position.y += 1;
            },
            (East, RightTurn) => {
                self.direction = North;
                self.position.y -= 1;
            },
            (North, Intersection) => {
                match self.at_intersection {
                    TurnLeft => {
                        self.direction = West;
                        self.position.x -= 1;
                    },
                    GoStraight => {
                        self.position.y -= 1;
                    },
                    TurnRight => {
                        self.direction = East;
                        self.position.x += 1;
                    },
                }
                self.at_intersection = self.at_intersection.next();
            },
            (South, Intersection) => {
                match self.at_intersection {
                    TurnLeft => {
                        self.direction = East;
                        self.position.x += 1;
                    },
                    GoStraight => {
                        self.position.y += 1;
                    },
                    TurnRight => {
                        self.direction = West;
                        self.position.x -= 1;
                    },
                }
                self.at_intersection = self.at_intersection.next();
            },
            (West, Intersection) => {
                match self.at_intersection {
                    TurnLeft => {
                        self.direction = South;
                        self.position.y += 1;
                    },
                    GoStraight => {
                        self.position.x -= 1;
                    },
                    TurnRight => {
                        self.direction = North;
                        self.position.y -= 1;
                    },
                }
                self.at_intersection = self.at_intersection.next();
            },
            (East, Intersection) => {
                match self.at_intersection {
                    TurnLeft => {
                        self.direction = North;
                        self.position.y -= 1;
                    },
                    GoStraight => {
                        self.position.x += 1;
                    },
                    TurnRight => {
                        self.direction = South;
                        self.position.y += 1;
                    },
                }
                self.at_intersection = self.at_intersection.next();
            },
            (dir, rail) => return IllegalMove(dir, rail),
        };
        Moved
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailKind {
    NorthSouth,
    WestEast,
    RightTurn,
    LeftTurn,
    Intersection,
}

impl Display for RailKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RailKind::*;
        let symbol = match *self {
            NorthSouth => "|",
            WestEast => "-",
            RightTurn => "/",
            LeftTurn => "\\",
            Intersection => "+",
        };
        f.write_str(symbol)
    }
}

impl From<Direction> for RailKind {
    fn from(direction: Direction) -> Self {
        use self::Direction::*;
        use self::RailKind::*;
        match direction {
            North => NorthSouth,
            East => WestEast,
            South => NorthSouth,
            West => WestEast,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tracks {
    rails: HashMap<Position, RailKind>,
}

impl Tracks {
    pub fn new() -> Self {
        Self {
            rails: HashMap::new(),
        }
    }

    pub fn rail(&self, position: Position) -> Option<&RailKind> {
        self.rails.get(&position)
    }

    pub fn area(&self) -> (Position, Position) {
        let mut top_left = Position::new(u32::MAX, u32::MAX);
        let mut bottom_right = Position::new(u32::MIN, u32::MIN);
        for pos in self.rails.keys() {
            if pos.x < top_left.x {
                top_left.x = pos.x;
            }
            if pos.x > bottom_right.x {
                bottom_right.x = pos.x;
            }
            if pos.y < top_left.y {
                top_left.y = pos.y;
            }
            if pos.y > bottom_right.y {
                bottom_right.y = pos.y;
            }
        }
        (top_left, bottom_right)
    }

    pub fn move_carts(&self, carts: &mut Vec<Cart>) -> MoveResult {
        let mut move_result = Moved;

        carts.sort_unstable_by_key(|cart| cart.position);

        let mut cart_positions: HashSet<_> = HashSet::from_iter(carts.iter().map(|c| c.position));

        let mut processed = 0;
        while processed < carts.len() {
            let mut collided = None;

            for cart in carts.iter_mut().skip(processed) {
                processed += 1;
                // move cart one step
                cart_positions.remove(&cart.position);
                let move_result = cart.move_one_step(self.rails[&cart.position]);
                if let IllegalMove(dir, rail) = move_result {
                    return IllegalMove(dir, rail);
                }
                // collision detection
                if !cart_positions.insert(cart.position) {
                    collided = Some(*cart);
                    break;
                }
            }

            // remove eventually collided carts
            if let Some(cart) = collided {
                let mut collided = Vec::with_capacity(2);
                if let Some(other_cart) = carts.iter().find_map(|c| {
                    if *c != cart && c.position == cart.position {
                        Some(*c)
                    } else {
                        None
                    }
                }) {
                    collided.push(other_cart);
                    collided.push(cart);

                    // if first collision remember collision for move result
                    if move_result == Moved {
                        move_result = Collision(cart, other_cart);
                    }
                }
                for cart in collided {
                    if let Some(index) = carts.iter().position(|c| *c == cart) {
                        if index < processed {
                            processed -= 1;
                        }
                        carts.remove(index);
                    }
                }
            }
        }

        move_result
    }
}

impl Index<Position> for Tracks {
    type Output = RailKind;

    fn index(&self, position: Position) -> &<Self as Index<Position>>::Output {
        &self.rails[&position]
    }
}

impl IndexMut<Position> for Tracks {
    fn index_mut(&mut self, position: Position) -> &mut <Self as Index<Position>>::Output {
        self.rails
            .get_mut(&position)
            .unwrap_or_else(|| panic!("no rail at position {}", position))
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CartsNTracks {
    num_carts: u8,
    carts: Vec<Cart>,
    tracks: Tracks,
}

impl Display for CartsNTracks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let area = self.tracks.area();
        let width = (area.1.y - area.0.y) as usize + 1;
        for y in area.0.y..=area.1.y {
            let mut line = String::with_capacity(width);
            for x in area.0.x..=area.1.x {
                let pos = Position::new(x, y);
                let symbol = self
                    .carts
                    .iter()
                    .find_map(|cart| {
                        if cart.position == pos {
                            let symbol = match cart.direction {
                                Direction::North => '^',
                                Direction::East => '>',
                                Direction::South => 'v',
                                Direction::West => '<',
                            };
                            Some(symbol)
                        } else {
                            None
                        }
                    })
                    .or_else(|| {
                        self.tracks.rail(pos).map(|rail| match *rail {
                            RailKind::NorthSouth => '|',
                            RailKind::WestEast => '-',
                            RailKind::RightTurn => '/',
                            RailKind::LeftTurn => '\\',
                            RailKind::Intersection => '+',
                        })
                    })
                    .unwrap_or(' ');
                line.push(symbol);
            }
            while let Some(chr) = line.pop() {
                if chr != ' ' {
                    line.push(chr);
                    break;
                }
            }
            line.push('\n');
            f.write_str(&line)?;
        }
        Ok(())
    }
}

impl CartsNTracks {
    pub fn new() -> Self {
        Self {
            num_carts: 0,
            carts: Vec::new(),
            tracks: Tracks::new(),
        }
    }

    pub fn carts(&self) -> &[Cart] {
        &self.carts
    }

    pub fn tracks(&self) -> &Tracks {
        &self.tracks
    }

    pub fn insert_cart(&mut self, position: Position, direction: Direction) {
        self.num_carts += 1;
        let cart = Cart::new(CartNo::new(self.num_carts), position, direction);
        self.carts.push(cart);
        self.insert_rail(position, direction.into());
    }

    pub fn remove_cart(&mut self, cart: &Cart) -> Option<Cart> {
        self.carts
            .iter()
            .position(|c| *c == *cart)
            .map(|idx| self.carts.remove(idx))
    }

    pub fn insert_rail(&mut self, position: Position, rail_kind: RailKind) {
        self.tracks.rails.insert(position, rail_kind);
    }

    pub fn move_carts(&mut self) -> MoveResult {
        self.tracks.move_carts(&mut self.carts)
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> CartsNTracks {
    let mut carts_n_tracks = CartsNTracks::new();

    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            let pos = Position::new(x as u32, y as u32);
            use self::Direction::*;
            use self::RailKind::*;
            match symbol {
                '-' => {
                    carts_n_tracks.insert_rail(pos, WestEast);
                },
                '|' => {
                    carts_n_tracks.insert_rail(pos, NorthSouth);
                },
                '/' => {
                    carts_n_tracks.insert_rail(pos, RightTurn);
                },
                '\\' => {
                    carts_n_tracks.insert_rail(pos, LeftTurn);
                },
                '+' => {
                    carts_n_tracks.insert_rail(pos, Intersection);
                },
                '^' => {
                    carts_n_tracks.insert_cart(pos, North);
                },
                '>' => {
                    carts_n_tracks.insert_cart(pos, East);
                },
                '<' => {
                    carts_n_tracks.insert_cart(pos, West);
                },
                'v' => {
                    carts_n_tracks.insert_cart(pos, South);
                },
                s if s.is_whitespace() => {},
                _ => panic!("unsupported character {:?} at {}:{}", symbol, y + 1, x + 1),
            }
        }
    }

    carts_n_tracks
}

#[aoc(day13, part1)]
pub fn location_of_first_crash(carts_n_tracks: &CartsNTracks) -> Position {
    let mut carts_n_tracks = carts_n_tracks.clone();
    let mut _step = 0;
    loop {
        _step += 1;
        let move_result = carts_n_tracks.move_carts();
        debug!("{:03}:\n{}", _step, carts_n_tracks);
        match move_result {
            Moved => {},
            Collision(cart, _) => return cart.position,
            IllegalMove(dir, rail) => panic!("illegal move in {} direction on rail {}", dir, rail),
        }
    }
}

#[aoc(day13, part2)]
pub fn location_of_last_cart(carts_n_tracks: &CartsNTracks) -> Position {
    let mut carts_n_tracks = carts_n_tracks.clone();
    let mut _step = 0;
    loop {
        _step += 1;
        let move_result = carts_n_tracks.move_carts();
        debug!("{:03}:\n{}", _step, carts_n_tracks);
        match move_result {
            Moved => {},
            Collision(_, _) => {
                if carts_n_tracks.carts().len() <= 1 {
                    return carts_n_tracks
                        .carts()
                        .get(0)
                        .unwrap_or_else(|| panic!("no cart left"))
                        .position;
                }
            },
            IllegalMove(dir, rail) => panic!("illegal move in {} direction on rail {}", dir, rail),
        }
    }
}

#[cfg(test)]
mod tests;
