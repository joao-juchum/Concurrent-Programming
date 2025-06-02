mod blocking_future;
pub mod caches;
mod evaluation;
mod game;
mod network;
mod utils;

/// Height of the game board
pub const HEIGHT: usize = 6;
/// Width of the game board
/// Should not be more than 25
pub const WIDTH: usize = 9;
/// Number of symbols to align to win
pub const POWER: usize = 4;

pub use blocking_future::BlockingFuture;
pub use evaluation::{AsyncEvaluator, EstimationResult, SyncEvaluator};
pub use game::play::Play;
pub use game::{End, Game, Player};
pub use network::{AIType, RemoteGame, Roles};
pub use utils::{
    get_user_commande, play_until_end, play_until_end_with_async, play_until_end_with_sync,
};

/// Package off all the robot players
pub mod evaluators {
    pub use crate::evaluation::{
        async_wrapper::BlockingTaskWrapper, min_max::MinMaxPolicy,
        min_max_cached::MinMaxPolicyCached, random_ai::RandomPolicy,
        threaded_wrapper::ThreadedPolicy,
    };
}

mod thread_pool;
