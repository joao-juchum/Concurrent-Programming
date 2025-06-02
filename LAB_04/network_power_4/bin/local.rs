use std::io::{BufWriter, stdout};

use network_power_4::{Game, get_user_commande};

fn main() {
    let mut game = Game::default();
    let out = stdout();
    let mut out = BufWriter::new(out);
    loop {
        game.render(&mut out);
        let column = get_user_commande();
        let res = game.play(column);
        match res {
            Ok(e) => {
                if let Some(e) = e {
                    println!("{e:?}");
                    break;
                }
            }
            Err(e) => println!("Erreur : {e:?}"),
        }
    }
    println!("Fin!");
}
