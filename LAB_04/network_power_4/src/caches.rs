//! Caches to store information
mod multi_thread;
mod single_thread;

use crate::{End, Player, game::board::Board};

pub use multi_thread::KnowledgeCacheMultiThread;
pub use single_thread::KnowledgeCacheSingleThread;

/// A cache implementation
pub trait KnowledgeCache {
    /// Lookup the already calculated ending from a `Board` and a `Player`, with the best column to play
    fn lookup(&self, board_state: Board, player: Player) -> Option<(usize, End)>;
    /// Store a newly calculated best move for a `Player` from a `Board`
    fn remember(
        &self,
        board_state: Board,
        player: Player,
        best_choice: usize,
        projected_ending: End,
    );
    /// Empty the cache
    fn clean(&mut self);
    /// Number of entry in a cache
    fn len(&self) -> usize;
    /// Is the cache empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
