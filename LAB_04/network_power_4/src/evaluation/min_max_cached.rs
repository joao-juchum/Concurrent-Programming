use log::trace;

use crate::{Play, Player, WIDTH, caches::KnowledgeCache, game::board::Board};

use super::{EstimationResult, SyncEvaluator};

/// A MinMax evaluator with a cache
pub struct MinMaxPolicyCached<C: KnowledgeCache> {
    max_depth: usize,
    knowledge_cache: C,
}

impl<C: KnowledgeCache + Default> MinMaxPolicyCached<C> {
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            knowledge_cache: C::default(),
        }
    }
}
impl<C: KnowledgeCache> MinMaxPolicyCached<C> {
    pub fn get_knowledge_size(&self) -> usize {
        self.knowledge_cache.len()
    }

    fn max(&self, board: &Board, player: Player, depth: usize) -> (usize, EstimationResult) {
        if depth == 0 {
            return (WIDTH / 2, EstimationResult::Partial(board.naive_eval()));
        };
        let legal_move = board.legal_moves(player);

        let move_evaluation: Vec<(usize, EstimationResult)> = legal_move
            .iter()
            .map(|(idx, b, e)| match e {
                Some(e) => {
                    trace!("Cache hit : {e:?}");
                    (*idx, EstimationResult::Full(*e))
                }
                None => match self.knowledge_cache.lookup(*b, player) {
                    None => {
                        let estimation = (*idx, self.max(b, player.other(), depth - 1).1);
                        if let EstimationResult::Full(e) = estimation.1 {
                            self.knowledge_cache.remember(*b, player, estimation.0, e);
                        };
                        estimation
                    }
                    Some(x) => (x.0, EstimationResult::Full(x.1)),
                },
            })
            .collect();
        EstimationResult::best_for(&move_evaluation, player)
    }
}

impl<C: KnowledgeCache> SyncEvaluator for MinMaxPolicyCached<C> {
    fn evaluate(
        &self,
        board: &crate::game::board::Board,
        player: crate::Player,
    ) -> (Play, EstimationResult) {
        let best = self.max(board, player, self.max_depth);
        (Play::try_from((best.0, player)).unwrap(), best.1)
    }
}

impl<C: KnowledgeCache + Clone> Clone for MinMaxPolicyCached<C> {
    fn clone(&self) -> Self {
        Self {
            max_depth: self.max_depth,
            knowledge_cache: self.knowledge_cache.clone(),
        }
    }
}
