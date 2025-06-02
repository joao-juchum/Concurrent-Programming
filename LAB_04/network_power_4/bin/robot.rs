use std::{
    io::Write,
    time::{Duration, Instant},
};

use clap::Parser;
use log::{error, info, warn};
use network_power_4::{
    RemoteGame, Roles, SyncEvaluator,
    caches::{KnowledgeCacheMultiThread, KnowledgeCacheSingleThread},
    evaluators::{MinMaxPolicy, MinMaxPolicyCached, ThreadedPolicy},
};
use tokio::runtime;

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    /// Role
    #[arg(value_enum)]
    role: Roles,

    /// Network adress
    remote_addr: String,

    /// Depth of the search
    depth: usize,

    #[clap(long, short, action)]
    /// Render the game
    render: bool,

    #[clap(long, short, action)]
    /// Ping to show that the program is alive
    alive: bool,

    #[clap(long, short, action)]
    /// Use thread parallelism
    thread: bool,

    #[clap(long, short, action)]
    /// Use a cache
    cache: bool,
}

fn main() {
    colog::init();
    let args = Cli::parse();

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        if args.alive {
            tokio::spawn(async {
                loop {
                    warn!("I'm alive");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            });
        }

        let mut game = match args.role {
            Roles::Client => RemoteGame::new_client(args.remote_addr).await,
            Roles::Host => RemoteGame::new_server(args.remote_addr).await,
        };
        info!("Player connected!");

        let evaluator: &mut dyn SyncEvaluator = if args.thread {
            if args.cache {
                &mut ThreadedPolicy::from(MinMaxPolicyCached::<KnowledgeCacheMultiThread>::new(
                    args.depth - 1,
                ))
            } else {
                &mut ThreadedPolicy::from(MinMaxPolicy::new(args.depth - 1))
            }
        } else if args.cache {
            &mut MinMaxPolicyCached::<KnowledgeCacheSingleThread>::new(args.depth)
        } else {
            &mut MinMaxPolicy::new(args.depth)
        };

        let mut buff = std::io::BufWriter::new(std::io::stdout());

        loop {
            if args.render {
                game.render(&mut buff);
                buff.flush().unwrap();
            }
            info!("Thinking...");
            let start = Instant::now();
            let (p, e) = evaluator.evaluate_game(&game.game());
            let end = Instant::now();
            info!("Think for {:} ms", (end - start).as_millis());
            info!("Playing {p:?}");
            info!("Estimation : {e:?}");
            let e = game.play(p.column()).await;
            match e {
                Ok(v) => match v {
                    Some(e) => {
                        if args.render {
                            game.render(&mut buff);
                            buff.flush().unwrap();
                        };
                        info!("{e:?}");
                        break;
                    }
                    None => continue,
                },
                Err(e) => {
                    error!("{e:?}");
                    panic!();
                }
            }
        }
    });
}
