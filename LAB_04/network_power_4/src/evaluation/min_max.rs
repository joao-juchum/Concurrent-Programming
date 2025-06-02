use crate::{Play, Player, WIDTH, game::board::Board};

use super::{EstimationResult, SyncEvaluator};

#[derive(Debug, Copy, Clone)]
/// A basic MinMaxing policy, with a fixed depth
///
/// Not optimised in any way
/// We might want to add :
/// - Caching,
/// - Multi-threading (see [`crate::evaluators::ThreadedPolicy`])
pub struct MinMaxPolicy {
    max_depth: usize,
}

impl MinMaxPolicy {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }
}
fn max(board: &Board, player: Player, depth: usize) -> (usize, EstimationResult) {
    if depth == 0 {
        return (WIDTH / 2, EstimationResult::Partial(board.naive_eval()));
    };
    let legal_move = board.legal_moves(player);

    let move_evaluation: Vec<(usize, EstimationResult)> = legal_move
        .into_iter()
        .map(|(idx, b, e)| match e {
            Some(e) => (idx, EstimationResult::Full(e)),
            None => (idx, max(&b, player.other(), depth - 1).1),
        })
        .collect();
    EstimationResult::best_for(&move_evaluation, player)
}

impl SyncEvaluator for MinMaxPolicy {
    fn evaluate(
        &self,
        board: &crate::game::board::Board,
        player: crate::Player,
    ) -> (Play, EstimationResult) {
        let best = max(board, player, self.max_depth);
        (Play::try_from((best.0, player)).unwrap(), best.1)
    }
}
