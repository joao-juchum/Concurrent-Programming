use serde::{Deserialize, Serialize};

use crate::{Player, WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// A play from a `Player`
pub struct Play {
    column: usize,
    player: Player,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfBound,
}

impl TryFrom<(usize, Player)> for Play {
    type Error = Error;

    fn try_from(value: (usize, Player)) -> Result<Self, Self::Error> {
        if value.0 >= WIDTH {
            Err(Error::OutOfBound)
        } else {
            Ok(Self {
                column: value.0,
                player: value.1,
            })
        }
    }
}

impl TryFrom<(usize, &Player)> for Play {
    type Error = Error;

    fn try_from(value: (usize, &Player)) -> Result<Self, Self::Error> {
        if value.0 >= WIDTH {
            Err(Error::OutOfBound)
        } else {
            Ok(Self {
                column: value.0,
                player: *value.1,
            })
        }
    }
}

impl Play {
    /// The column of the play
    pub fn column(&self) -> usize {
        self.column
    }
    /// The player of the play
    pub fn player(&self) -> Player {
        self.player
    }
}
