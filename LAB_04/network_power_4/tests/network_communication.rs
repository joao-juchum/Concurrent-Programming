use std::time::Duration;

use futures::pin_mut;
use network_power_4::{RemoteGame, evaluators::RandomPolicy, play_until_end};
use tokio::time::Instant;

#[test]
fn init_remote_game() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let addr = "localhost:4545";

        let init_conn = futures::future::join(
            async {
                let game = RemoteGame::new_server(addr).await;
                play_until_end(game).await;
            },
            async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                let client = RemoteGame::new_client(addr).await;
                play_until_end(client).await
            },
        );

        tokio::time::timeout_at(Instant::now() + Duration::from_secs(1), init_conn)
            .await
            .unwrap();
    })
}
