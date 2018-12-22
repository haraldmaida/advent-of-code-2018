//! # Day 15: Beverage Bandits
//!
//! Having perfected their hot chocolate, the Elves have a new problem: the
//! Goblins that live in these caves will do anything to steal it. Looks like
//! they're here for a fight.
//!
//! You scan the area, generating a map of the walls (#), open cavern (.), and
//! starting position of every Goblin (G) and Elf (E) (your puzzle input).
//!
//! Combat proceeds in rounds; in each round, each unit that is still alive
//! takes a turn, resolving all of its actions before the next unit's turn
//! begins. On each unit's turn, it tries to move into range of an enemy (if it
//! isn't already) and then attack (if it is in range).
//!
//! All units are very disciplined and always follow very strict combat rules.
//! Units never move or attack diagonally, as doing so would be dishonorable.
//! When multiple choices are equally valid, ties are broken in reading order:
//! top-to-bottom, then left-to-right. For instance, the order in which units
//! take their turns within a round is the reading order of their starting
//! positions in that round, regardless of the type of unit or whether other
//! units have moved after the round started. For example:
//!
//! ```text
//!                  would take their
//! These units:   turns in this order:
//!   #######           #######
//!   #.G.E.#           #.1.2.#
//!   #E.G.E#           #3.4.5#
//!   #.G.E.#           #.6.7.#
//!   #######           #######
//! ```
//!
//! Each unit begins its turn by identifying all possible targets (enemy units).
//! If no targets remain, combat ends.
//!
//! Then, the unit identifies all of the open squares (.) that are in range of
//! each target; these are the squares which are adjacent (immediately up, down,
//! left, or right) to any target and which aren't already occupied by a wall or
//! another unit. Alternatively, the unit might already be in range of a target.
//! If the unit is not already in range of a target, and there are no open
//! squares which are in range of a target, the unit ends its turn.
//!
//! If the unit is already in range of a target, it does not move, but continues
//! its turn with an attack. Otherwise, since it is not in range of a target, it
//! moves.
//!
//! To move, the unit first considers the squares that are in range and
//! determines which of those squares it could reach in the fewest steps. A step
//! is a single movement to any adjacent (immediately up, down, left, or right)
//! open (.) square. Units cannot move into walls or other units. The unit does
//! this while considering the current positions of units and does not do any
//! prediction about where units will be later. If the unit cannot reach (find
//! an open path to) any of the squares that are in range, it ends its turn. If
//! multiple squares are in range and tied for being reachable in the fewest
//! steps, the step which is first in reading order is chosen. For example:
//!
//! ```text
//! Targets:      In range:     Reachable:    Nearest:      Chosen:
//! #######       #######       #######       #######       #######
//! #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
//! #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
//! #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
//! #######       #######       #######       #######       #######
//! ```
//!
//! In the above scenario, the Elf has three targets (the three Goblins):
//!
//! * Each of the Goblins has open, adjacent squares which are in range (marked
//!   with a ? on the map).
//! * Of those squares, four are reachable (marked @); the other two (on the
//!   right) would require moving through a wall or unit to reach.
//! * Three of these reachable squares are nearest, requiring the fewest steps
//!   (only 2) to reach (marked !).
//! * Of those, the square which is first in reading order is chosen (+).
//!
//! The unit then takes a single step toward the
//! chosen square along the shortest path to that square. If multiple steps
//! would put the unit equally closer to its destination, the unit chooses the
//! step which is first in reading order. (This requires knowing when there is
//! more than one shortest path so that you can consider the first step of each
//! such path.) For example:
//!
//! ```text
//! In range:     Nearest:      Chosen:       Distance:     Step:
//! #######       #######       #######       #######       #######
//! #.E...#       #.E...#       #.E...#       #4E212#       #..E..#
//! #...?.#  -->  #...!.#  -->  #...+.#  -->  #32101#  -->  #.....#
//! #..?G?#       #..!G.#       #...G.#       #432G2#       #...G.#
//! #######       #######       #######       #######       #######
//! ```
//!
//! The Elf sees three squares in range of a target (?), two of which are
//! nearest (!), and so the first in reading order is chosen (+). Under
//! "Distance", each open square is marked with its distance from the
//! destination square; the two squares to which the Elf could move on this turn
//! (down and to the right) are both equally good moves and would leave the Elf
//! 2 steps from being in range of the Goblin. Because the step which is first
//! in reading order is chosen, the Elf moves right one square.
//!
//! Here's a larger example of movement:
//!
//! ```text
//! Initially:
//! #########
//! #G..G..G#
//! #.......#
//! #.......#
//! #G..E..G#
//! #.......#
//! #.......#
//! #G..G..G#
//! #########
//!
//! After 1 round:
//! #########
//! #.G...G.#
//! #...G...#
//! #...E..G#
//! #.G.....#
//! #.......#
//! #G..G..G#
//! #.......#
//! #########
//!
//! After 2 rounds:
//! #########
//! #..G.G..#
//! #...G...#
//! #.G.E.G.#
//! #.......#
//! #G..G..G#
//! #.......#
//! #.......#
//! #########
//!
//! After 3 rounds:
//! #########
//! #.......#
//! #..GGG..#
//! #..GEG..#
//! #G..G...#
//! #......G#
//! #.......#
//! #.......#
//! #########
//! ```
//!
//! Once the Goblins and Elf reach the positions above, they all are either in
//! range of a target or cannot find any square in range of a target, and so
//! none of the units can move until a unit dies.
//!
//! After moving (or if the unit began its turn in range of a target), the unit
//! attacks.
//!
//! To attack, the unit first determines all of the targets that are in range of
//! it by being immediately adjacent to it. If there are no such targets, the
//! unit ends its turn. Otherwise, the adjacent target with the fewest hit
//! points is selected; in a tie, the adjacent target with the fewest hit points
//! which is first in reading order is selected.
//!
//! The unit deals damage equal to its attack power to the selected target,
//! reducing its hit points by that amount. If this reduces its hit points to 0
//! or fewer, the selected target dies: its square becomes . and it takes no
//! further turns.
//!
//! Each unit, either Goblin or Elf, has 3 attack power and starts with 200 hit
//! points.
//!
//! For example, suppose the only Elf is about to attack:
//!
//! ```text
//!        HP:            HP:
//! G....  9       G....  9
//! ..G..  4       ..G..  4
//! ..EG.  2  -->  ..E..
//! ..G..  2       ..G..  2
//! ...G.  1       ...G.  1
//! ```
//!
//! The "HP" column shows the hit points of the Goblin to the left in the
//! corresponding row. The Elf is in range of three targets: the Goblin above it
//! (with 4 hit points), the Goblin to its right (with 2 hit points), and the
//! Goblin below it (also with 2 hit points). Because three targets are in
//! range, the ones with the lowest hit points are selected: the two Goblins
//! with 2 hit points each (one to the right of the Elf and one below the Elf).
//! Of those, the Goblin first in reading order (the one to the right of the
//! Elf) is selected. The selected Goblin's hit points (2) are reduced by the
//! Elf's attack power (3), reducing its hit points to -1, killing it.
//!
//! After attacking, the unit's turn ends. Regardless of how the unit's turn
//! ends, the next unit in the round takes its turn. If all units have taken
//! turns in this round, the round ends, and a new round begins.
//!
//! The Elves look quite outnumbered. You need to determine the outcome of the
//! battle: the number of full rounds that were completed (not counting the
//! round in which combat ends) multiplied by the sum of the hit points of all
//! remaining units at the moment combat ends. (Combat only ends when a unit
//! finds no targets during its turn.)
//!
//! Below is an entire sample combat. Next to each map, each row's units' hit
//! points are listed from left to right.
//!
//! ```text
//! Initially:
//! #######
//! #.G...#   G(200)
//! #...EG#   E(200), G(200)
//! #.#.#G#   G(200)
//! #..G#E#   G(200), E(200)
//! #.....#
//! #######
//!
//! After 1 round:
//! #######
//! #..G..#   G(200)
//! #...EG#   E(197), G(197)
//! #.#G#G#   G(200), G(197)
//! #...#E#   E(197)
//! #.....#
//! #######
//!
//! After 2 rounds:
//! #######
//! #...G.#   G(200)
//! #..GEG#   G(200), E(188), G(194)
//! #.#.#G#   G(194)
//! #...#E#   E(194)
//! #.....#
//! #######
//!
//! Combat ensues; eventually, the top Elf dies:
//!
//! After 23 rounds:
//! #######
//! #...G.#   G(200)
//! #..G.G#   G(200), G(131)
//! #.#.#G#   G(131)
//! #...#E#   E(131)
//! #.....#
//! #######
//!
//! After 24 rounds:
//! #######
//! #..G..#   G(200)
//! #...G.#   G(131)
//! #.#G#G#   G(200), G(128)
//! #...#E#   E(128)
//! #.....#
//! #######
//!
//! After 25 rounds:
//! #######
//! #.G...#   G(200)
//! #..G..#   G(131)
//! #.#.#G#   G(125)
//! #..G#E#   G(200), E(125)
//! #.....#
//! #######
//!
//! After 26 rounds:
//! #######
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(122)
//! #...#E#   E(122)
//! #..G..#   G(200)
//! #######
//!
//! After 27 rounds:
//! #######
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(119)
//! #...#E#   E(119)
//! #...G.#   G(200)
//! #######
//!
//! After 28 rounds:
//! #######
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(116)
//! #...#E#   E(113)
//! #....G#   G(200)
//! #######
//!
//! More combat ensues; eventually, the bottom Elf dies:
//!
//! After 47 rounds:
//! #######
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(59)
//! #...#.#
//! #....G#   G(200)
//! #######
//! ```
//!
//! Before the 48th round can finish, the top-left Goblin finds that there are
//! no targets remaining, and so combat ends. So, the number of full rounds that
//! were completed is 47, and the sum of the hit points of all remaining units
//! is 200+131+59+200 = 590. From these, the outcome of the battle is 47 * 590 =
//! 27730.
//!
//! Here are a few example summarized combats:
//!
//! ```text
//! #######       #######
//! #G..#E#       #...#E#   E(200)
//! #E#E.E#       #E#...#   E(197)
//! #G.##.#  -->  #.E##.#   E(185)
//! #...#E#       #E..#E#   E(200), E(200)
//! #...E.#       #.....#
//! #######       #######
//!
//! Combat ends after 37 full rounds
//! Elves win with 982 total hit points left
//! Outcome: 37 * 982 = 36334
//!
//! #######       #######
//! #E..EG#       #.E.E.#   E(164), E(197)
//! #.#G.E#       #.#E..#   E(200)
//! #E.##E#  -->  #E.##.#   E(98)
//! #G..#.#       #.E.#.#   E(200)
//! #..E#.#       #...#.#
//! #######       #######
//!
//! Combat ends after 46 full rounds
//! Elves win with 859 total hit points left
//! Outcome: 46 * 859 = 39514
//!
//! #######       #######
//! #E.G#.#       #G.G#.#   G(200), G(98)
//! #.#G..#       #.#G..#   G(200)
//! #G.#.G#  -->  #..#..#
//! #G..#.#       #...#G#   G(95)
//! #...E.#       #...G.#   G(200)
//! #######       #######
//!
//! Combat ends after 35 full rounds
//! Goblins win with 793 total hit points left
//! Outcome: 35 * 793 = 27755
//!
//! #######       #######
//! #.E...#       #.....#
//! #.#..G#       #.#G..#   G(200)
//! #.###.#  -->  #.###.#
//! #E#G#G#       #.#.#.#
//! #...#G#       #G.G#G#   G(98), G(38), G(200)
//! #######       #######
//!
//! Combat ends after 54 full rounds
//! Goblins win with 536 total hit points left
//! Outcome: 54 * 536 = 28944
//!
//! #########       #########
//! #G......#       #.G.....#   G(137)
//! #.E.#...#       #G.G#...#   G(200), G(200)
//! #..##..G#       #.G##...#   G(200)
//! #...##..#  -->  #...##..#
//! #...#...#       #.G.#...#   G(200)
//! #.G...G.#       #.......#
//! #.....G.#       #.......#
//! #########       #########
//!
//! Combat ends after 20 full rounds
//! Goblins win with 937 total hit points left
//! Outcome: 20 * 937 = 18740
//! ```
//!
//! What is the outcome of the combat described in your puzzle input?
//!
//! ## Part 2
//!
//! According to your calculations, the Elves are going to lose badly. Surely,
//! you won't mess up the timeline too much if you give them just a little
//! advanced technology, right?
//!
//! You need to make sure the Elves not only win, but also suffer no losses:
//! even the death of a single Elf is unacceptable.
//!
//! However, you can't go too far: larger changes will be more likely to
//! permanently alter spacetime.
//!
//! So, you need to find the outcome of the battle in which the Elves have the
//! lowest integer attack power (at least 4) that allows them to win without a
//! single death. The Goblins always have an attack power of 3.
//!
//! In the first summarized example above, the lowest attack power the Elves
//! need to win without losses is 15:
//!
//! ```text
//! #######       #######
//! #.G...#       #..E..#   E(158)
//! #...EG#       #...E.#   E(14)
//! #.#.#G#  -->  #.#.#.#
//! #..G#E#       #...#.#
//! #.....#       #.....#
//! #######       #######
//!
//! Combat ends after 29 full rounds
//! Elves win with 172 total hit points left
//! Outcome: 29 * 172 = 4988
//! ```
//!
//! In the second example above, the Elves need only 4 attack power:
//!
//! ```text
//! #######       #######
//! #E..EG#       #.E.E.#   E(200), E(23)
//! #.#G.E#       #.#E..#   E(200)
//! #E.##E#  -->  #E.##E#   E(125), E(200)
//! #G..#.#       #.E.#.#   E(200)
//! #..E#.#       #...#.#
//! #######       #######
//!
//! Combat ends after 33 full rounds
//! Elves win with 948 total hit points left
//! Outcome: 33 * 948 = 31284
//! ```
//!
//! In the third example above, the Elves need 15 attack power:
//!
//! ```text
//! #######       #######
//! #E.G#.#       #.E.#.#   E(8)
//! #.#G..#       #.#E..#   E(86)
//! #G.#.G#  -->  #..#..#
//! #G..#.#       #...#.#
//! #...E.#       #.....#
//! #######       #######
//!
//! Combat ends after 37 full rounds
//! Elves win with 94 total hit points left
//! Outcome: 37 * 94 = 3478
//! ```
//!
//! In the fourth example above, the Elves need 12 attack power:
//!
//! ```text
//! #######       #######
//! #.E...#       #...E.#   E(14)
//! #.#..G#       #.#..E#   E(152)
//! #.###.#  -->  #.###.#
//! #E#G#G#       #.#.#.#
//! #...#G#       #...#.#
//! #######       #######
//!
//! Combat ends after 39 full rounds
//! Elves win with 166 total hit points left
//! Outcome: 39 * 166 = 6474
//! ```
//!
//! In the last example above, the lone Elf needs 34 attack power:
//!
//! ```text
//! #########       #########
//! #G......#       #.......#
//! #.E.#...#       #.E.#...#   E(38)
//! #..##..G#       #..##...#
//! #...##..#  -->  #...##..#
//! #...#...#       #...#...#
//! #.G...G.#       #.......#
//! #.....G.#       #.......#
//! #########       #########
//!
//! Combat ends after 30 full rounds
//! Elves win with 38 total hit points left
//! Outcome: 30 * 38 = 1140
//! ```
//!
//! After increasing the Elves' attack power until it is just barely enough for
//! them to win without any Elves dying, what is the outcome of the combat
//! described in your puzzle input?
//!
//! [Advent of Code 2018 - Day 15](https://adventofcode.com/2018/day/15)

use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    iter::FromIterator,
    marker::PhantomData,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use self::FightResult::*;
use self::Move::*;
use self::Tile::*;

pub trait Unit {
    const NAME: &'static str;
}

pub type IdValue = u32;
pub const MIN_ID_VAL: u32 = std::u32::MIN;
pub const MAX_ID_VAL: u32 = std::u32::MAX;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id<T: Unit> {
    _unit: PhantomData<T>,
    value: IdValue,
}

impl<T: Unit> Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id({})", self.value)
    }
}

impl<T: Unit> Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::derive_hash_xor_eq))]
impl<T: Unit> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        <T as Unit>::NAME.hash(state);
        self.value.hash(state);
    }
}

impl<T: Unit> Id<T> {
    pub fn new(value: IdValue) -> Self {
        Self {
            _unit: PhantomData,
            value,
        }
    }

    pub fn val(self) -> IdValue {
        self.value
    }
}

#[derive(Debug)]
pub struct IdSequence<T: Unit> {
    _unit: PhantomData<T>,
    step: IdValue,
    last: IdValue,
}

impl<T: Unit> Default for IdSequence<T> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T: Unit> IdSequence<T> {
    pub fn new(step: IdValue) -> Self {
        Self {
            _unit: PhantomData,
            step,
            last: 0,
        }
    }

    pub fn next_val(&mut self) -> Id<T> {
        self.last += 1;
        Id::new(self.last)
    }
}

pub type Coord = usize;
pub const MAX_COORD: Coord = std::usize::MAX;
pub const MIN_COORD: Coord = std::usize::MIN;

pub type Distance = usize;
pub const MAX_DISTANCE: Distance = std::usize::MAX;

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

impl Position {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn north(self) -> Option<Self> {
        if self.y > MIN_COORD {
            Some(Position {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    pub fn south(self) -> Option<Self> {
        if self.y < MAX_COORD {
            Some(Position {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    pub fn east(self) -> Option<Self> {
        if self.x < MAX_COORD {
            Some(Position {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    pub fn west(self) -> Option<Self> {
        if self.x > MIN_COORD {
            Some(Position {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    pub fn manhattan_distance(self, other: Position) -> Distance {
        let dx = if self.x < other.x {
            other.x - self.x
        } else {
            self.x - other.x
        };

        let dy = if self.y < other.y {
            other.y - self.y
        } else {
            self.y - other.y
        };

        dx + dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Up => "\u{2191}",
            Down => "\u{2193}",
            Left => "\u{2190}",
            Right => "\u{2192}",
        };
        f.write_str(symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HitPoints(i32);

impl Display for HitPoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl HitPoints {
    pub const ZERO: HitPoints = HitPoints(0);

    pub fn val(self) -> i32 {
        self.0
    }
}

impl Default for HitPoints {
    fn default() -> Self {
        HitPoints(200)
    }
}

impl Add for HitPoints {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        HitPoints(self.0 + rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttackPower(i32);

impl Display for AttackPower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AttackPower {
    pub fn new(value: i32) -> Self {
        AttackPower(value)
    }

    pub fn val(self) -> i32 {
        self.0
    }
}

impl Default for AttackPower {
    fn default() -> Self {
        AttackPower(3)
    }
}

impl AddAssign<i32> for AttackPower {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}

impl SubAssign<i32> for AttackPower {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs;
    }
}

impl Sub<AttackPower> for HitPoints {
    type Output = HitPoints;

    fn sub(self, rhs: AttackPower) -> <Self as Sub<AttackPower>>::Output {
        HitPoints(self.0 - rhs.0)
    }
}

impl SubAssign<AttackPower> for HitPoints {
    fn sub_assign(&mut self, rhs: AttackPower) {
        self.0 -= rhs.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf {
    id: Id<Elf>,
    hit_points: HitPoints,
}

impl Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E{}", self.id)
    }
}

impl Unit for Elf {
    const NAME: &'static str = stringify!(Elf);
}

impl HasId for Elf {
    fn id(self) -> Id<Self> {
        self.id
    }
}

impl<'a> HasId<Elf> for &'a Elf {
    fn id(self) -> Id<Elf> {
        self.id
    }
}

impl HasHitPoints for Elf {
    fn hit_points(&self) -> HitPoints {
        self.hit_points
    }

    fn hit_points_mut(&mut self) -> &mut HitPoints {
        &mut self.hit_points
    }
}

impl Elf {
    pub fn new(id: Id<Self>) -> Self {
        Self {
            id,
            hit_points: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Goblin {
    id: Id<Goblin>,
    hit_points: HitPoints,
}

impl Display for Goblin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "G{}", self.id)
    }
}

impl Unit for Goblin {
    const NAME: &'static str = stringify!(Goblin);
}

impl HasId for Goblin {
    fn id(self) -> Id<Self> {
        self.id
    }
}

impl<'a> HasId<Goblin> for &'a Goblin {
    fn id(self) -> Id<Goblin> {
        self.id
    }
}

impl HasHitPoints for Goblin {
    fn hit_points(&self) -> HitPoints {
        self.hit_points
    }

    fn hit_points_mut(&mut self) -> &mut HitPoints {
        &mut self.hit_points
    }
}

impl Goblin {
    pub fn new(id: Id<Self>) -> Self {
        Self {
            id,
            hit_points: Default::default(),
        }
    }
}

pub trait HasId<T = Self>
where
    T: Unit,
    Self: Sized,
{
    fn id(self) -> Id<T>;
}

pub trait HasHitPoints {
    fn hit_points(&self) -> HitPoints;

    fn hit_points_mut(&mut self) -> &mut HitPoints;
}

pub trait WithHitPoints<T: Unit> {
    fn hit_points(&self, id: Id<T>) -> HitPoints;

    fn hit_points_mut(&mut self, id: Id<T>) -> &mut HitPoints;
}

pub trait WithPositioned<T: Unit> {
    fn position_of(&self, id: Id<T>) -> Option<Position>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    OpenCavern,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            OpenCavern => ".",
            Wall => "#",
        };
        f.write_str(symbol)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cave(HashSet<Position>);

impl Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (top_left, bottom_right) = self.area();
        for y in top_left.y..=bottom_right.y {
            let mut formatted = String::with_capacity(40);
            for x in top_left.x..=bottom_right.x {
                let position = Position { x, y };
                let symbol = match self.tile(position) {
                    OpenCavern => '.',
                    Wall => '#',
                };
                formatted.push(symbol);
            }
            formatted.push('\n');
            f.write_str(&formatted)?;
        }
        Ok(())
    }
}

impl Cave {
    pub fn with_walls(wall_positions: impl IntoIterator<Item = Position>) -> Self {
        Cave(HashSet::from_iter(wall_positions.into_iter()))
    }

    pub fn tile(&self, position: Position) -> Tile {
        if self.0.contains(&position) {
            Wall
        } else {
            OpenCavern
        }
    }

    pub fn area(&self) -> (Position, Position) {
        let mut top_left = Position::MAX;
        let mut bottom_right = Position::MIN;
        for wall in &self.0 {
            if wall.x < top_left.x {
                top_left.x = wall.x;
            }
            if wall.y < top_left.y {
                top_left.y = wall.y
            }
            if wall.x > bottom_right.x {
                bottom_right.x = wall.x;
            }
            if wall.y > bottom_right.y {
                bottom_right.y = wall.y;
            }
        }
        (top_left, bottom_right)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Vertex {
    previous: Option<Position>,
    local: Distance,
    heuristic: Distance,
    global: Distance,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            previous: None,
            local: MAX_DISTANCE,
            heuristic: MAX_DISTANCE,
            global: MAX_DISTANCE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FightResult {
    WinnerElves(HitPoints),
    WinnerGoblins(HitPoints),
    Tie,
    Ongoing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Combat {
    cave: Cave,
    elves: HashMap<Position, Elf>,
    goblins: HashMap<Position, Goblin>,
    rounds: u32,
    elves_attack_power: AttackPower,
    goblins_attack_power: AttackPower,
}

impl Display for Combat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (top_left, bottom_right) = self.cave.area();
        for y in top_left.y..=bottom_right.y {
            let mut formatted = String::with_capacity(40);
            for x in top_left.x..=bottom_right.x {
                let position = Position { x, y };
                let symbol = self
                    .elves
                    .keys()
                    .find_map(|pos| if *pos == position { Some('E') } else { None })
                    .or_else(|| {
                        self.goblins.keys().find_map(|pos| {
                            if *pos == position {
                                Some('G')
                            } else {
                                None
                            }
                        })
                    })
                    .unwrap_or_else(|| match self.cave.tile(position) {
                        Wall => '#',
                        _ => '.',
                    });
                formatted.push(symbol);
            }
            formatted.push('\n');
            f.write_str(&formatted)?;
        }
        Ok(())
    }
}

impl WithPositioned<Elf> for Combat {
    fn position_of(&self, id: Id<Elf>) -> Option<Position> {
        self.elves.iter().find_map(|(position, elf)| {
            if elf.id() == id {
                Some(*position)
            } else {
                None
            }
        })
    }
}

impl WithPositioned<Goblin> for Combat {
    fn position_of(&self, id: Id<Goblin>) -> Option<Position> {
        self.goblins.iter().find_map(|(position, goblin)| {
            if goblin.id() == id {
                Some(*position)
            } else {
                None
            }
        })
    }
}

impl Combat {
    pub fn new(
        walls: impl IntoIterator<Item = Position>,
        elves: impl IntoIterator<Item = (Position, Elf)>,
        goblins: impl IntoIterator<Item = (Position, Goblin)>,
    ) -> Self {
        let elves = HashMap::from_iter(elves.into_iter());
        let goblins = HashMap::from_iter(goblins.into_iter());

        Self {
            cave: Cave::with_walls(walls),
            elves,
            goblins,
            rounds: 0,
            elves_attack_power: AttackPower::default(),
            goblins_attack_power: AttackPower::default(),
        }
    }

    pub fn elves(&self) -> impl Iterator<Item = &Elf> {
        self.elves.values()
    }

    pub fn elf(&self, position: Position) -> Option<&Elf> {
        self.elves.get(&position)
    }

    pub fn goblins(&self) -> impl Iterator<Item = &Goblin> {
        self.goblins.values()
    }

    pub fn goblin(&self, position: Position) -> Option<&Goblin> {
        self.goblins.get(&position)
    }

    pub fn rounds(&self) -> u32 {
        self.rounds
    }

    pub fn fight(&mut self) -> FightResult {
        'fighting: loop {
            debug!("combat:\n{}", self);
            let fight = self.fight_one_round();
            if fight != Ongoing {
                debug!("finished combat:\n{}", self);
                break 'fighting fight;
            }
        }
    }

    pub fn n_fights(&mut self, number_of_rounds: u32) -> FightResult {
        let mut last_result = Ongoing;
        for _ in 0..number_of_rounds {
            last_result = self.fight_one_round();
        }
        debug!("|> Elves: {:?}", self.elves);
        debug!("|> Goblins: {:?}", self.goblins);
        last_result
    }

    pub fn fight_one_round(&mut self) -> FightResult {
        let mut units = Vec::from_iter(
            self.elves
                .keys()
                .cloned()
                .chain(self.goblins.keys().cloned()),
        );
        let num_units = units.len();

        units.sort_unstable();

        let mut num_acting_units = 0;
        for unit in units {
            if self.elves.contains_key(&unit) {
                if let Some(goblin) = enemy_to_attack(unit, &self.goblins) {
                    attack_enemy(goblin, self.elves_attack_power, &mut self.goblins);
                } else if let Some(new_position) = self.move_elf_towards_goblins(unit) {
                    if let Some(goblin) = enemy_to_attack(new_position, &self.goblins) {
                        attack_enemy(goblin, self.elves_attack_power, &mut self.goblins);
                    }
                }
            } else if self.goblins.contains_key(&unit) {
                if let Some(elf) = enemy_to_attack(unit, &self.elves) {
                    attack_enemy(elf, self.goblins_attack_power, &mut self.elves);
                } else if let Some(new_position) = self.move_goblin_towards_elfs(unit) {
                    if let Some(elf) = enemy_to_attack(new_position, &self.elves) {
                        attack_enemy(elf, self.goblins_attack_power, &mut self.elves);
                    }
                }
            } else {
                //panic!("WTF! unit must be either an elf or a goblin: {:?}", unit);
            };
            num_acting_units += 1;
            if self.elves.is_empty() || self.goblins.is_empty() {
                break;
            }
        }

        if num_acting_units == num_units {
            self.rounds += 1;
        }

        self.status()
    }

    fn status(&self) -> FightResult {
        if self.elves.is_empty() {
            if self.goblins.is_empty() {
                Tie
            } else {
                let remaining_hitpoints = self
                    .goblins
                    .values()
                    .fold(HitPoints::ZERO, |sum, e| sum + e.hit_points);
                WinnerGoblins(remaining_hitpoints)
            }
        } else if self.goblins.is_empty() {
            let remaining_hitpoints = self
                .elves
                .values()
                .fold(HitPoints::ZERO, |sum, e| sum + e.hit_points);
            WinnerElves(remaining_hitpoints)
        } else {
            Ongoing
        }
    }

    fn move_goblin_towards_elfs(&mut self, goblin: Position) -> Option<Position> {
        let eventually_add_adjacents_of_enemy =
            |enemy: Position, targets: &mut HashSet<Position>| {
                let mut eventually_add_target = |maybe_target| {
                    if let Some(target) = maybe_target {
                        if target != goblin && self.is_free_position(target) {
                            targets.insert(target);
                        }
                    }
                };
                eventually_add_target(enemy.north());
                eventually_add_target(enemy.west());
                eventually_add_target(enemy.east());
                eventually_add_target(enemy.south());
            };

        let mut targets = HashSet::with_capacity(4);
        self.elves
            .keys()
            .for_each(|enemy| eventually_add_adjacents_of_enemy(*enemy, &mut targets));
        debug!(
            "trying to move goblin {} torwards possible targets: {:?}",
            goblin, targets
        );

        self.path_to_nearest_target(goblin, targets)
            .and_then(|path| path.get(1).cloned())
            .and_then(|step1| move_unit(goblin, step1, &mut self.goblins))
    }

    fn move_elf_towards_goblins(&mut self, elf: Position) -> Option<Position> {
        let eventually_add_adjacents_of_enemy =
            |enemy: Position, targets: &mut HashSet<Position>| {
                let mut eventually_add_target = |maybe_target| {
                    if let Some(target) = maybe_target {
                        if target != elf && self.is_free_position(target) {
                            targets.insert(target);
                        }
                    }
                };
                eventually_add_target(enemy.north());
                eventually_add_target(enemy.west());
                eventually_add_target(enemy.east());
                eventually_add_target(enemy.south());
            };

        let mut targets = HashSet::with_capacity(4);
        self.goblins
            .keys()
            .for_each(|enemy| eventually_add_adjacents_of_enemy(*enemy, &mut targets));
        debug!(
            "trying to move elf {} torwards possible targets: {:?}",
            elf, targets
        );

        self.path_to_nearest_target(elf, targets)
            .and_then(|path| path.get(1).cloned())
            .and_then(|step1| move_unit(elf, step1, &mut self.elves))
    }

    fn path_to_nearest_target(
        &self,
        unit: Position,
        targets: impl IntoIterator<Item = Position>,
    ) -> Option<Vec<Position>> {
        targets
            .into_iter()
            .map(|target| self.shortest_path(unit, target))
            .filter(|path| !path.is_empty())
            .min_by(|path1, path2| {
                let distance_cmp = path1.len().cmp(&path2.len());
                if distance_cmp == Ordering::Equal {
                    path1.last().cmp(&path2.last())
                } else {
                    distance_cmp
                }
            })
    }

    fn is_free_position(&self, position: Position) -> bool {
        self.cave.tile(position) == OpenCavern
            && !self.elves.contains_key(&position)
            && !self.goblins.contains_key(&position)
    }

    fn adjacent_free_positions(&self, position: Position, target: Position) -> Vec<Position> {
        let mut adjacent_positions = Vec::with_capacity(4);
        let mut eventually_push = |position| {
            if let Some(pos) = position {
                if pos == target || self.is_free_position(pos) {
                    adjacent_positions.push(pos);
                }
            }
        };

        eventually_push(position.north());
        eventually_push(position.west());
        eventually_push(position.east());
        eventually_push(position.south());

        adjacent_positions
    }

    fn shortest_path(&self, start: Position, end: Position) -> Vec<Position> {
        debug!("searching shortest path from {} to {}", start, end);
        let heuristic = |a_pos: Position, target: Position| a_pos.manhattan_distance(target);

        let mut found_path_start = None;
        let mut vertices: HashMap<Position, Vertex> = HashMap::with_capacity(16);
        let mut open: Vec<Position> = Vec::with_capacity(4);
        open.push(end);

        let mut distance = 0;
        {
            let mut curr_vertex = Vertex::default();
            curr_vertex.local = distance;
            curr_vertex.heuristic = heuristic(end, start);
            curr_vertex.global = curr_vertex.heuristic;
            vertices.insert(end, curr_vertex);
        }

        'exploring: while let Some(curr_position) = open.pop() {
            debug!("current position: {:?}", curr_position);
            distance += 1;
            for a_position in self.adjacent_free_positions(curr_position, start) {
                debug!("| exploring vertex: {:?}", a_position);
                let a_vertex: &mut Vertex = vertices.entry(a_position).or_insert_with(|| {
                    open.push(a_position);
                    Vertex::default()
                });
                if a_vertex.heuristic == MAX_DISTANCE {
                    a_vertex.heuristic = heuristic(a_position, start);
                }
                if distance < a_vertex.local {
                    a_vertex.local = distance;
                    a_vertex.global = a_vertex.local + a_vertex.heuristic;
                    a_vertex.previous = Some(curr_position);
                } else if distance == a_vertex.local && a_vertex.previous.unwrap() < curr_position {
                    a_vertex.previous = Some(curr_position);
                }
            }
            if curr_position == start {
                found_path_start = Some(curr_position);
                debug!("found target position: {:?}", found_path_start);
                break 'exploring;
            }
            open.sort_unstable_by(|p1, p2| {
                let global_cmp = vertices[p2].global.cmp(&vertices[p1].global);
                if global_cmp == Ordering::Equal {
                    p2.cmp(p1)
                } else {
                    global_cmp
                }
            });
        }
        debug!("resulting vertices: {:?}", vertices);

        let mut path: Vec<Position> = Vec::with_capacity(8);
        let mut previous = found_path_start;
        while let Some(position) = previous {
            path.push(position);
            previous = vertices.get(&position).and_then(|v| v.previous);
        }
        debug!("found path: {:?}", path);

        path
    }
}

fn move_unit<T>(
    unit: Position,
    target: Position,
    unit_positions: &mut HashMap<Position, T>,
) -> Option<Position> {
    unit_positions.remove(&unit).and_then(|unit| {
        unit_positions.insert(target, unit);
        Some(target)
    })
}

fn enemy_to_attack<T>(unit: Position, enemies: &HashMap<Position, T>) -> Option<Position>
where
    T: HasHitPoints,
{
    let mut adjacent_positions = HashSet::with_capacity(4);
    let mut eventually_add_position = |maybe_position| {
        if let Some(position) = maybe_position {
            adjacent_positions.insert(position);
        }
    };
    eventually_add_position(unit.north());
    eventually_add_position(unit.west());
    eventually_add_position(unit.east());
    eventually_add_position(unit.south());

    enemies
        .iter()
        .filter(|(pos, _)| adjacent_positions.contains(pos))
        .min_by(|(pos1, unit1), (pos2, unit2)| {
            let hit_cmp = unit1.hit_points().cmp(&unit2.hit_points());
            if hit_cmp == Ordering::Equal {
                pos1.cmp(pos2)
            } else {
                hit_cmp
            }
        })
        .map(|(pos, _)| *pos)
}

fn attack_enemy<T>(
    position: Position,
    attack_power: AttackPower,
    enemies: &mut HashMap<Position, T>,
) -> Option<(Position, T)>
where
    T: HasHitPoints + Debug,
{
    let enemy_died = if let Some(enemy) = enemies.get_mut(&position) {
        *enemy.hit_points_mut() -= attack_power;
        debug!("attacked {:?}", enemy);
        enemy.hit_points() <= HitPoints::ZERO
    } else {
        false
    };
    if enemy_died {
        enemies.remove(&position).map(|enemy| (position, enemy))
    } else {
        None
    }
}

#[aoc_generator(day15)]
pub fn parse(input: &str) -> Combat {
    let mut elf_id_seq = IdSequence::default();
    let mut goblin_id_seq = IdSequence::default();
    let mut walls = HashSet::with_capacity(16);
    let mut elves = HashMap::new();
    let mut goblins = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            let position = Position { x, y };
            match chr {
                '.' => {},
                '#' => {
                    walls.insert(position);
                },
                'E' => {
                    let id = elf_id_seq.next_val();
                    elves.insert(position, Elf::new(id));
                },
                'G' => {
                    let id = goblin_id_seq.next_val();
                    goblins.insert(position, Goblin::new(id));
                },
                s if s.is_whitespace() => {},
                _ => {
                    panic!("unexpected character {} at {}:{}", chr, y, x);
                },
            }
        }
    }

    Combat::new(walls, elves, goblins)
}

#[aoc(day15, part1)]
pub fn fight(combat_map: &Combat) -> i32 {
    let mut combat = combat_map.clone();
    calculate_outcome(combat.fight(), combat.rounds())
}

fn calculate_outcome(fight_result: FightResult, rounds: u32) -> i32 {
    match fight_result {
        WinnerElves(remaining_hitpoints) => {
            debug!(
                "Elves win after {} rounds with {} total hit points left!",
                rounds, remaining_hitpoints
            );
            remaining_hitpoints.val() * rounds as i32
        },
        WinnerGoblins(remaining_hitpoints) => {
            debug!(
                "Goblins win after {} rounds with {} total hit points left!",
                rounds, remaining_hitpoints
            );
            remaining_hitpoints.val() * rounds as i32
        },
        Tie => {
            debug!("The combat ended with a tie after {} rounds", rounds);
            0
        },
        Ongoing => unreachable!(),
    }
}

#[aoc(day15, part2)]
pub fn fake_fight(combat_map: &Combat) -> i32 {
    let combat = combat_map.clone();
    let (_, rounds, fight_result) = least_attach_power_for_elves_to_win(combat);
    calculate_outcome(fight_result, rounds)
}

fn least_attach_power_for_elves_to_win(initial_combat: Combat) -> (AttackPower, u32, FightResult) {
    let orig_num_elves = initial_combat.elves.len();
    let mut combat = initial_combat.clone();
    let mut faked_attack_power = AttackPower(4);
    combat.elves_attack_power = faked_attack_power;
    let fight = 'fighting: loop {
        debug!("combat:\n{}", combat);
        let fight = combat.fight_one_round();
        let lost_lives = orig_num_elves - combat.elves.len();
        if lost_lives > 0 {
            faked_attack_power += 1;
            combat = initial_combat.clone();
            combat.elves_attack_power = faked_attack_power;
            debug!(
                "elves lost {} lives -> trying with fake power of {}",
                lost_lives, combat.elves_attack_power
            );
        }
        if fight != Ongoing {
            debug!("finished combat:\n{}", combat);
            break 'fighting fight;
        }
    };
    info!(
        "elves win with faked attack power of {}",
        combat.elves_attack_power
    );
    (combat.elves_attack_power, combat.rounds(), fight)
}

#[cfg(test)]
mod tests;
