//! Game logic

pub mod board;
pub mod play;

use std::io::BufWriter;

use board::Board;
use play::Play;
use serde::{Deserialize, Serialize};

use crate::RemoteGame;

const VERTICAL_SEPARATOR: &str = "|";
const HORIZONTAL_SEPARATOR: &str = "-";
const CROSS_SEPARATOR: &str = "+";
const FILLER: &str = " ";

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, PartialOrd, Ord)]
/// An enum for the players
pub enum Player {
    /// First player
    FIRST,
    /// Second player
    SECOND,
}

impl Player {
    /// Get the other player
    pub fn other(&self) -> Self {
        match self {
            Player::FIRST => Player::SECOND,
            Player::SECOND => Player::FIRST,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// The state of a game
pub struct Game {
    current_board: Board,
    to_play: Player,
    history: Vec<Play>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A possible ending of a game
pub enum End {
    /// The player has won
    Win { player: Player },
    /// There is no more possible play, no one has won
    Stall,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_board: Board::default(),
            to_play: Player::FIRST,
            history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlayError {
    OutOfBound,
    FullCollumn,
}

impl From<play::Error> for GamePlayError {
    fn from(value: play::Error) -> Self {
        match value {
            play::Error::OutOfBound => Self::OutOfBound,
        }
    }
}

impl From<board::ApplyError> for GamePlayError {
    fn from(value: board::ApplyError) -> Self {
        match value {
            board::ApplyError::ColumnFull => Self::FullCollumn,
        }
    }
}

impl From<Vec<Play>> for Game {
    fn from(value: Vec<Play>) -> Self {
        let a = Board::default();
        let b = a.apply_sequence(&value);
        let to_play = if (value.len() % 2) == 1 {
            Player::SECOND
        } else {
            Player::FIRST
        };
        Self {
            current_board: b.0,
            to_play,
            history: value,
        }
    }
}

impl From<RemoteGame> for Game {
    fn from(value: RemoteGame) -> Self {
        let plays = value.history();
        Self::from(plays)
    }
}

impl Game {
    /// Play in the column `col`
    pub fn play(&mut self, col: usize) -> Result<Option<End>, GamePlayError> {
        let p: Play = Play::try_from((col, &self.to_play))?;
        let current = &self.current_board;
        let (next, winner) = current.apply(p)?;
        self.current_board = next;
        self.to_play = self.to_play.other();
        self.history.push(p);
        Ok(winner)
    }
    /// Render the game into the buffer
    pub fn render<T: std::io::Write>(&self, buff: &mut BufWriter<T>) {
        self.current_board.render(buff);
    }
    /// Get the history of play of the game
    pub fn history(&self) -> Vec<Play> {
        self.history.clone()
    }
    /// Get the current ending of the game
    pub fn end(&self) -> Option<End> {
        self.current_board.end()
    }
    /// Get the next player to play
    pub fn next_to_play(&self) -> Player {
        self.to_play
    }
    /// Get the current state of the board
    pub fn board(&self) -> Board {
        self.current_board
    }
}
