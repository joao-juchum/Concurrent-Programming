use std::sync::Arc;

use futures::future::join_all;

use once_cell::sync::Lazy;

use crate::{Play, Player, game::board::Board};

use crate::thread_pool::ThreadPool;

use super::{AsyncEvaluator, EstimationResult, SyncEvaluator};

/// A wrapper around a [`SyncEvaluator`] to make it [`AsyncEvaluator`] by launching async task
///
/// We could use either out [`crate::BlockingFuture`] or [`tokio::task::spawn_blocking`]
#[derive(Debug, Clone)]
pub struct BlockingTaskWrapper<T: SyncEvaluator> {
    evaluator: Arc<T>,
}

// Static ThreadPool shared by all evaluators (new)
static POOL: Lazy<ThreadPool> = Lazy::new(|| ThreadPool::new(4));


impl<T: SyncEvaluator> From<T> for BlockingTaskWrapper<T> {
    fn from(value: T) -> Self {
        Self {
            evaluator: value.into(),
        }
    }
}

impl<T: SyncEvaluator + Send + Sync + Clone + 'static> AsyncEvaluator for BlockingTaskWrapper<T> {
    async fn evaluate(
        &self,
        board: std::sync::Arc<Board>,
        player: Player,
    ) -> (Play, EstimationResult) {
        let legal_move = board.legal_moves(player);

        let move_evaluation: Vec<_> = legal_move
            .into_iter()
            .map(|(idx, b, e)| {
                let player = player.other();
                let evaluator = self.evaluator.clone();

                // Application without ThreadPool
                //
                // tokio::task::spawn_blocking(move || match e {
                //     Some(e) => (idx, EstimationResult::Full(e)),
                //     None => (idx, evaluator.evaluate(&b, player).1),
                // })

                POOL.execute(move || match e {
                    Some(e) => (idx, EstimationResult::Full(e)),
                    None => (idx, evaluator.evaluate(&b, player).1),
                })

            })
            .collect();

        let results: Vec<_> = join_all(move_evaluation)
            .await
            .into_iter()
            .map(|res| {
                let (idx, est) = res; //.expect("Thread panicked"); //remove expect because ThreadPool handles panic safety internally (new)
                (Play::try_from((idx, player)).expect("Invalid move"), est)
            })
            .collect();

        EstimationResult::best_for(&results, player)
    }
}
