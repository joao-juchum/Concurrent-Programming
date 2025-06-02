use std::io::{BufWriter, stdout};

use clap::Parser;
use log::info;
use network_power_4::{RemoteGame, Roles, get_user_commande};
use tokio::runtime;

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    /// Role
    #[arg(value_enum)]
    role: Roles,

    /// Network adress
    remote_addr: String,
}

fn main() {
    colog::init();
    let args = Cli::parse();

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let mut game = match args.role {
            Roles::Client => RemoteGame::new_client(args.remote_addr).await,
            Roles::Host => RemoteGame::new_server(args.remote_addr).await,
        };
        let out = stdout();
        let mut out = BufWriter::new(out);
        game.render(&mut out);
        loop {
            let mut e;
            loop {
                let c = get_user_commande();
                e = game.play(c).await;
                if e.is_ok() {
                    break;
                }
            }
            game.render(&mut out);
            if e.unwrap().is_some() {
                info!("{e:?}");
                break;
            };
        }
        print!("Fin!")
    })
}
