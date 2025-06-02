use std::sync::{Arc, Mutex};

use super::{EstimationResult, SyncEvaluator};
use crate::game::board::Board;
use crate::{Play, Player};
use log::error;
use rand::prelude::*;
use rand::rngs::ThreadRng;

/// A evaluator playing at random
pub struct RandomPolicy {
    rand: Arc<Mutex<ThreadRng>>,
}

impl Default for RandomPolicy {
    fn default() -> Self {
        RandomPolicy {
            rand: Arc::from(Mutex::from(rand::rng())),
        }
    }
}

impl SyncEvaluator for RandomPolicy {
    fn evaluate(&self, board: &Board, player: Player) -> (Play, EstimationResult) {
        if board.end().is_some() {
            error!("Nothing to play !");
            panic!();
        } else {
            let moves = board.legal_moves(player);
            let c = moves.choose(&mut self.rand.lock().unwrap()).unwrap();
            (
                Play::try_from((c.0, player)).unwrap(),
                EstimationResult::Partial(0.0),
            )
        }
    }
}
