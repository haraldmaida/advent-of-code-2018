//! # Day 9: Marble Mania
//!
//! You talk to the Elves while you wait for your navigation system to
//! initialize. To pass the time, they introduce you to their favorite marble
//! game.
//!
//! The Elves play this game by taking turns arranging the marbles in a circle
//! according to very particular rules. The marbles are numbered starting with 0
//! and increasing by 1 until every marble has a number.
//!
//! First, the marble numbered 0 is placed in the circle. At this point, while
//! it contains only a single marble, it is still a circle: the marble is both
//! clockwise from itself and counter-clockwise from itself. This marble is
//! designated the current marble.
//!
//! Then, each Elf takes a turn placing the lowest-numbered remaining marble
//! into the circle between the marbles that are 1 and 2 marbles clockwise of
//! the current marble. (When the circle is large enough, this means that there
//! is one marble between the marble that was just placed and the current
//! marble.) The marble that was just placed then becomes the current marble.
//!
//! However, if the marble that is about to be placed has a number which is a
//! multiple of 23, something entirely different happens. First, the current
//! player keeps the marble they would have placed, adding it to their score. In
//! addition, the marble 7 marbles counter-clockwise from the current marble is
//! removed from the circle and also added to the current player's score. The
//! marble located immediately clockwise of the marble that was removed becomes
//! the new current marble.
//!
//! For example, suppose there are 9 players. After the marble with value 0 is
//! placed in the middle, each player (shown in square brackets) takes a turn.
//! The result of each of those turns would produce circles of marbles like
//! this, where clockwise is to the right and the resulting current marble is in
//! parentheses:
//!
//! ```text
//! [-] (0)
//! [1]  0 (1)
//! [2]  0 (2) 1
//! [3]  0  2  1 (3)
//! [4]  0 (4) 2  1  3
//! [5]  0  4  2 (5) 1  3
//! [6]  0  4  2  5  1 (6) 3
//! [7]  0  4  2  5  1  6  3 (7)
//! [8]  0 (8) 4  2  5  1  6  3  7
//! [9]  0  8  4 (9) 2  5  1  6  3  7
//! [1]  0  8  4  9  2(10) 5  1  6  3  7
//! [2]  0  8  4  9  2 10  5(11) 1  6  3  7
//! [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
//! [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
//! [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
//! [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
//! [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
//! [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
//! [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
//! [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
//! [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
//! [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
//! ```
//!
//! The goal is to be the player with the highest score after the last marble is
//! used up. Assuming the example above ends after the marble numbered 25, the
//! winning score is 23+9=32 (because player 5 kept marble 23 and removed marble
//! 9, while no other player got any points in this very short example game).
//!
//! Here are a few more examples:
//!
//! * 10 players; last marble is worth 1618 points: high score is 8317
//! * 13 players; last marble is worth 7999 points: high score is 146373
//! * 17 players; last marble is worth 1104 points: high score is 2764
//! * 21 players; last marble is worth 6111 points: high score is 54718
//! * 30 players; last marble is worth 5807 points: high score is 37305
//!
//! What is the winning Elf's score?
//!
//! ## Part 2
//!
//! Amused by the speed of your answer, the Elves are curious:
//!
//! What would the new winning Elf's score be if the number of the last marble
//! were 100 times larger?
//!
//! [Advent of Code 2018 - Day 9](https://adventofcode.com/2018/day/9)

use std::{
    collections::HashMap,
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Add, AddAssign, Index},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerNr(u32);

impl PlayerNr {
    pub const MIN: PlayerNr = PlayerNr(1);
    pub const MAX: PlayerNr = PlayerNr(999);

    pub fn new(val: u32) -> Self {
        PlayerNr(val)
    }

    pub fn val(self) -> u32 {
        self.0
    }
}

impl Display for PlayerNr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Score(u32);

impl Score {
    pub const ZERO: Score = Score(0);
    pub const ONE: Score = Score(1);

    pub fn new(val: u32) -> Self {
        Score(val)
    }

    pub fn val(self) -> u32 {
        self.0
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

impl Add<u32> for Score {
    type Output = Score;

    fn add(self, rhs: u32) -> Self::Output {
        Score(self.0 + rhs)
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl AddAssign<u32> for Score {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Marble(u32);

impl Marble {
    pub const ZERO: Marble = Marble(0);
    pub const ONE: Marble = Marble(1);

    pub fn new(val: u32) -> Self {
        Marble(val)
    }

    pub fn val(self) -> u32 {
        self.0
    }
}

impl Display for Marble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarbleGame {
    rules: MarbleRules,
    num_marbles: u32,
    num_players: u32,
}

impl AsRef<MarbleGame> for MarbleGame {
    fn as_ref(&self) -> &MarbleGame {
        self
    }
}

impl MarbleGame {
    pub fn new(num_marbles: u32, num_players: u32) -> Self {
        Self {
            rules: MarbleRules,
            num_marbles,
            num_players,
        }
    }

    pub fn runner(self) -> GameRunner {
        GameRunner::new(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarbleRules;

impl MarbleRules {
    pub fn is_special(self, marble: Marble) -> bool {
        marble.val() % 23 == 0
    }

    pub fn next_marble(self, marble: Marble) -> Marble {
        Marble(marble.val() + 1)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Ring<T> {
    list: Vec<T>,
}

impl<T> Ring<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: Vec::with_capacity(capacity),
        }
    }

    pub fn index_of(&self, item: &T) -> Option<usize> {
        self.list
            .iter()
            .enumerate()
            .find_map(|(idx, elem)| if elem == item { Some(idx) } else { None })
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn insert(&mut self, index: usize, item: T) {
        self.list.insert(index, item);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.list.remove(index)
    }
}

impl<T> AsRef<[T]> for Ring<T> {
    fn as_ref(&self) -> &[T] {
        &self.list
    }
}

impl<T> AsMut<[T]> for Ring<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.list
    }
}

impl<T> Index<usize> for Ring<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameRunner {
    game: MarbleGame,
    scores: HashMap<PlayerNr, Score>,
    ring: Ring<Marble>,
    current_index: usize,
    current_marble: Marble,
    current_player: PlayerNr,
}

impl GameRunner {
    pub fn new(game: MarbleGame) -> Self {
        Self {
            game,
            scores: Self::initial_scores(game.num_players),
            ring: Self::initial_ring(game.num_marbles),
            current_index: 0,
            current_marble: Marble::ZERO,
            current_player: PlayerNr(0),
        }
    }

    fn initial_ring(num_marbles: u32) -> Ring<Marble> {
        let mut ring = Ring::with_capacity(num_marbles as usize);
        ring.insert(0, Marble::ZERO);
        ring
    }

    fn initial_scores(num_players: u32) -> HashMap<PlayerNr, Score> {
        HashMap::from_iter((1..=num_players).map(|nr| (PlayerNr::new(nr), Score::ZERO)))
    }

    pub fn marbles(&self) -> &[Marble] {
        self.ring.as_ref()
    }

    pub fn score(&self, player_nr: PlayerNr) -> Option<Score> {
        self.scores.get(&player_nr).map(ToOwned::to_owned)
    }

    pub fn current_marble(&self) -> Marble {
        self.current_marble
    }

    pub fn current_player(&self) -> PlayerNr {
        self.current_player
    }

    pub fn next_player(&mut self) {
        if self.current_player.0 == self.game.num_players {
            self.current_player = PlayerNr(1);
        } else {
            self.current_player.0 += 1;
        }
    }

    pub fn next_marble(&mut self) {
        self.current_marble.0 += 1;
    }

    pub fn finish(mut self) -> HashMap<PlayerNr, Score> {
        while let Some(_) = self.next() {}
        self.scores
    }
}

impl Iterator for GameRunner {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_marble.val() == self.game.num_marbles {
            return None;
        }
        self.next_player();
        self.next_marble();
        if self.game.rules.is_special(self.current_marble) {
            if self.current_index >= 7 {
                self.current_index -= 7
            } else {
                self.current_index += self.ring.len() - 7
            };
            let removed_marble = self.ring.remove(self.current_index);
            let new_score = self.current_marble.val() + removed_marble.val();
            *self
                .scores
                .entry(self.current_player)
                .or_insert(Score::ZERO) += new_score;
            Some(())
        } else {
            let ring_len = self.ring.len();
            self.current_index += 2;
            if self.current_index > ring_len {
                self.current_index -= ring_len;
            }
            self.ring.insert(self.current_index, self.current_marble);
            Some(())
        }
    }
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> MarbleGame {
    let line =
        input.trim().lines().next().unwrap_or_else(|| {
            panic!("what? there is not a single line in the input: {:?}", input)
        });

    let mut words = line.trim().split(' ');

    let num_players = words.next().unwrap().parse().unwrap();
    let num_marbles = words.nth(5).unwrap().parse().unwrap();

    MarbleGame::new(num_marbles, num_players)
}

#[aoc(day9, part1)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
pub fn marble_highscore(marble_game: &MarbleGame) -> Score {
    let runner = marble_game.runner();
    runner
        .finish()
        .into_iter()
        .map(|(_, score)| score)
        .max()
        .expect("there should be any score because it is initialized for all players")
}

#[aoc(day9, part2)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
pub fn marble_highscore_100(marble_game: &MarbleGame) -> Score {
    let marble_game = MarbleGame::new(marble_game.num_marbles * 100, marble_game.num_players);
    let runner = marble_game.runner();
    runner
        .finish()
        .into_iter()
        .map(|(_, score)| score)
        .max()
        .expect("there should be any score because it is initialized for all players")
}

#[cfg(test)]
mod tests;
