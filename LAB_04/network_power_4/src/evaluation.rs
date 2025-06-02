//! Evaluator logic

use std::{ops::Neg, sync::Arc};

use crate::{
    Game, Play, Player,
    game::{End, board::Board},
};

pub mod async_wrapper;
pub mod min_max;
pub mod min_max_cached;
pub mod random_ai;
pub mod threaded_wrapper;

#[derive(Debug, Clone, Copy)]
/// Estimation of the ending from a position.
pub enum EstimationResult {
    /// The estimation can't determine if the position is winning or losing.
    ///
    /// Positive value mean that the `FIRST` player has an advantage, and negative for the `SECOND` player.
    Partial(f64),
    /// The estimation can determine the best best plau for this position.
    ///
    /// The game will end as `End`.
    Full(End),
}

impl EstimationResult {
    /// Compare to estimation to get the best for a player
    ///
    /// ```rs
    /// let w1 = EstimationResult::Full(End::Win(Player::FIRST));
    /// let w2 = EstimationResult::Full(End::Win(Player::SECOND));
    /// let s = EstimationResult::Full(End::Win(Player::SECOND));
    /// let a1 = EstimationResult::Partial(1.0);
    /// let a2 = EstimationResult::Partial(1.0);
    ///
    /// assert_eq!(w1.is_better(s, Player::FIRST), true);
    /// assert_eq!(w2.is_better(s, Player::FIRST), false);
    /// assert_eq!(s.is_better(a1, Player::FIRST), false);
    /// assert_eq!(s.is_better(a2, Player::FIRST), true);
    /// ```
    pub fn is_better(&self, e2: &EstimationResult, player: &Player) -> bool {
        match player {
            Player::FIRST => e2 > self,
            Player::SECOND => e2 < self,
        }
    }
    /// Get the best move from a list for a given player.
    ///
    /// ```
    /// use network_power_4::{EstimationResult, Player};
    /// let moves = [
    ///     (0, EstimationResult::Partial(0.1)),
    ///     (1, EstimationResult::Partial(101.345)),
    ///     (5, EstimationResult::Partial(-3.2))];
    /// let moves : &[(u8, EstimationResult)] = &moves[..];
    /// assert_eq!(EstimationResult::best_for(moves, Player::FIRST).0, 1);
    /// assert_eq!(EstimationResult::best_for(moves, Player::SECOND).0, 5);
    /// ```
    pub fn best_for<T: Copy>(
        inputs: &[(T, EstimationResult)],
        player: Player,
    ) -> (T, EstimationResult) {
        match player {
            Player::FIRST => *inputs.iter().max_by_key(|e| e.1).unwrap(),
            Player::SECOND => *inputs.iter().min_by_key(|e| e.1).unwrap(),
        }
    }
    pub fn into_partial(&self) -> Self {
        match *self {
            EstimationResult::Partial(_) => *self,
            EstimationResult::Full(e) => match e {
                End::Stall => EstimationResult::Partial(0.0),
                End::Win { player } => match player {
                    Player::FIRST => EstimationResult::Partial(f64::INFINITY),
                    Player::SECOND => EstimationResult::Partial(-f64::INFINITY),
                },
            },
        }
    }
}

impl Neg for EstimationResult {
    type Output = EstimationResult;
    fn neg(self) -> Self::Output {
        match self {
            EstimationResult::Full(e) => match e {
                End::Stall => EstimationResult::Full(End::Stall),
                End::Win { player } => EstimationResult::Full(End::Win {
                    player: player.other(),
                }),
            },
            EstimationResult::Partial(f) => EstimationResult::Partial(-f),
        }
    }
}

impl From<EstimationResult> for f64 {
    fn from(value: EstimationResult) -> Self {
        match value.into_partial() {
            EstimationResult::Partial(f) => f,
            EstimationResult::Full(_) => panic!(),
        }
    }
}
impl From<&EstimationResult> for f64 {
    fn from(value: &EstimationResult) -> Self {
        match value.into_partial() {
            EstimationResult::Partial(f) => f,
            EstimationResult::Full(_) => panic!(),
        }
    }
}

impl Eq for EstimationResult {}

impl PartialEq for EstimationResult {
    fn eq(&self, other: &Self) -> bool {
        let a: f64 = self.into();
        let b: f64 = other.into();
        a == b
    }
}

impl PartialOrd for EstimationResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EstimationResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a: f64 = self.into();
        let b: f64 = other.into();
        if a < b {
            std::cmp::Ordering::Less
        } else if b < a {
            std::cmp::Ordering::Greater
        } else if a == b {
            std::cmp::Ordering::Equal
        } else {
            panic!()
        }
    }
}

/// Evaluator that can give a recomendation for a play for a state of the game
///
/// This one has a blocking interface
pub trait SyncEvaluator {
    /// Return the estimated best play for the player `player` in the state `board`
    fn evaluate(&self, board: &Board, player: Player) -> (Play, EstimationResult);

    /// Return the estimated best play for the player `player` for the game state
    fn evaluate_game(&self, game: &Game) -> (Play, EstimationResult) {
        self.evaluate(&game.board(), game.next_to_play())
    }
}

/// Evaluator that can give a recomendation for a play for a state of the game
///
/// This one has a async interface
pub trait AsyncEvaluator {
    /// Return the estimated best play for the player `player` in the state `board`
    fn evaluate(
        &self,
        board: Arc<Board>,
        player: Player,
    ) -> impl Future<Output = (Play, EstimationResult)>;

    /// Return the estimated best play for the player `player` for the game state
    fn evaluate_game(&self, game: &Game) -> impl Future<Output = (Play, EstimationResult)> {
        self.evaluate(Arc::from(game.board()), game.next_to_play())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ESTIMATIONS: [EstimationResult; 6] = [
        EstimationResult::Full(End::Win {
            player: Player::FIRST,
        }),
        EstimationResult::Full(End::Win {
            player: Player::SECOND,
        }),
        EstimationResult::Full(End::Stall),
        EstimationResult::Partial(1.7),
        EstimationResult::Partial(2.2),
        EstimationResult::Partial(-0.1),
    ];
    #[test]
    fn is_better_eq_comp() {
        for a in ESTIMATIONS {
            for b in ESTIMATIONS {
                assert_eq!((&a).is_better(&b, &Player::FIRST), a < b);
            }
        }
    }
}
