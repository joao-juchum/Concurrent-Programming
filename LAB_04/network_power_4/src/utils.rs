use std::io::stdin;

use crate::{AsyncEvaluator, RemoteGame, SyncEvaluator, evaluators::RandomPolicy};

/// Loop until the user give a correct input and convert it to `usize`.
pub fn get_user_commande() -> usize {
    let column;
    loop {
        let mut user_input = String::default();
        stdin().read_line(&mut user_input).unwrap();
        user_input = String::from(user_input.trim());
        column = match string_to_ord(user_input.as_str()) {
            Some(u) => u,
            None => {
                println!("Erreur : No value readable");
                continue;
            }
        };
        break;
    }
    column - ('a' as usize)
}

fn string_to_ord(s: &str) -> Option<usize> {
    let c = s.chars().next();
    c.map(|c| c as usize)
}

/// Play a remote game util its end with the provided `AsyncEvaluator`
pub async fn play_until_end_with_async<T: AsyncEvaluator>(game: RemoteGame, policy: T) {
    let mut game = game;
    loop {
        let e = game
            .play(policy.evaluate_game(&game.game()).await.0.column())
            .await;
        let e = e.unwrap();
        if e.is_some() {
            break;
        }
    }
}
/// Play a remote game util its end with the provided `SyncEvaluator`
pub async fn play_until_end_with_sync<T: SyncEvaluator>(game: RemoteGame, policy: T) {
    let mut game = game;
    loop {
        let e = game
            .play(policy.evaluate_game(&game.game()).0.column())
            .await;
        let e = e.unwrap();
        if e.is_some() {
            break;
        }
    }
}

pub async fn play_until_end(game: RemoteGame) {
    let policy = RandomPolicy::default();
    play_until_end_with_sync(game, policy).await;
}
