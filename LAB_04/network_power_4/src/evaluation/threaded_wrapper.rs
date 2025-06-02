use std::sync::Arc;

use crate::Play;

use super::{EstimationResult, SyncEvaluator};

/// A wrapper around a `SyncEvaluator` to make it multi-treaded
#[derive(Debug, Clone)]
pub struct ThreadedPolicy<T: SyncEvaluator> {
    eval: Arc<T>,
}

impl<T: SyncEvaluator> From<T> for ThreadedPolicy<T> {
    fn from(value: T) -> Self {
        Self { eval: value.into() }
    }
}

impl<T: SyncEvaluator + Sync + Send + Clone + 'static> SyncEvaluator for ThreadedPolicy<T> {
    fn evaluate(
        &self,
        board: &crate::game::board::Board,
        player: crate::Player,
    ) -> (Play, EstimationResult) {
        // Get all possible column indices
        let columns: Vec<usize> = (0..crate::WIDTH).collect();

        // Create one thread per legal move
        let handles: Vec<_> = columns
            .into_iter()
            .filter_map(|col| {
                // Try to construct a valid play
                let play = Play::try_from((col, player)).ok()?;

                // Clone the board
                let board_clone = *board;

                // Try to apply the move to get a valid new board
                let new_board = match board_clone.apply(play) {
                    Ok((b, _)) => b,
                    Err(_) => return None,
                };

                let eval_clone = self.eval.clone();
                let opponent = player.other();

                // Spawn a thread to evaluate this move
                Some(std::thread::spawn(move || {
                    let (_, score) = eval_clone.evaluate(&new_board, opponent);
                    (play, -score)
                }))
            })
            .collect();

        // Join all threads and collect the results
        let mut results = Vec::new();
        for handle in handles {
            let result = handle.join().expect("Thread panicked");
            results.push(result);
        }

        // Return the best move
        EstimationResult::best_for(&results, player)
    }
}
