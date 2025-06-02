use crate::{End, Player, game::board::Board};
use std::{cell::RefCell, collections::BTreeMap};

use super::KnowledgeCache;

#[derive(Debug, Default, Clone)]
/// A non-thread safe implementation of cache
pub struct KnowledgeCacheSingleThread {
    inner: RefCell<BTreeMap<(Board, Player), (usize, End)>>,
}

impl KnowledgeCache for KnowledgeCacheSingleThread {
    fn lookup(&self, board_state: Board, player: Player) -> Option<(usize, End)> {
        self.inner.borrow().get(&(board_state, player)).copied()
    }
    fn remember(
        &self,
        board_state: Board,
        player: Player,
        best_choice: usize,
        projected_ending: End,
    ) {
        self.inner
            .borrow_mut()
            .insert((board_state, player), (best_choice, projected_ending));
    }
    fn clean(&mut self) {
        self.inner.get_mut().clear();
    }
    fn len(&self) -> usize {
        self.inner.borrow().len()
    }
}
