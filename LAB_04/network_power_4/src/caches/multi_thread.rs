use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};

use super::KnowledgeCache;
use crate::{End, Player, game::board::Board};

// To simplify the warning (warning: very complex type used. Consider factoring parts into `type` definitions) from clippy
type SharedCache = Arc<RwLock<BTreeMap<(Board, Player), (usize, End)>>>;

#[derive(Debug, Clone)]
/// A thread safe implementation of the caches
pub struct KnowledgeCacheMultiThread {
    // Internal cache protected by RwLock for concurrent access
    // inner: Arc<RwLock<BTreeMap<(Board, Player), (usize, End)>>>, // too complex to clippy
    inner: SharedCache,
}

impl KnowledgeCache for KnowledgeCacheMultiThread {
    fn lookup(&self, board_state: Board, player: Player) -> Option<(usize, End)> {
        // Acquire read access to the cache
        let map = self.inner.read();
        map.get(&(board_state, player)).copied()
    }

    fn remember(
        &self,
        board_state: Board,
        player: Player,
        best_choice: usize,
        projected_ending: End,
    ) {
        // Acquire write access and insert the result
        let mut map = self.inner.write();
        map.insert((board_state, player), (best_choice, projected_ending));
    }

    fn len(&self) -> usize {
        // Get length with read access
        self.inner.read().len()
    }

    fn clean(&mut self) {
        // Clear cache with write access
        self.inner.write().clear();
    }
}

impl Default for KnowledgeCacheMultiThread {
    fn default() -> Self {
        // Initialize with empty BTreeMap inside Arc<RwLock<>>
        Self {
            inner: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
