use console::Term;

use crate::{creatures::{get_new_player, player::Player}, data::{self, db}};

pub fn load_save() -> Player {
    let term: Term = Term::stdout();
    let saves = data::db::db::get_players();
    let mut player: Player;

    if saves.is_err() {
        // If player doesnt exists, ask for his name and initialize new player
        player = get_new_player().unwrap();

    } else {
        // If player exists ask to create new one or not
        term.write_line("Would you like to create new save? (y/n)").unwrap();

        let answer = term.read_char().unwrap();

        term.clear_screen().unwrap();

        if answer == 'y' {
            player = get_new_player().unwrap();
            player.id = saves.unwrap().len() as u8;
        } else {
            term.write_line("Select save: ").unwrap();

            for (_, player) in saves.unwrap().iter().enumerate() {
                term.write_line(&format!("{}. {}", player.0 + 1, player.1)).unwrap();
            }

            let index: usize = term.read_line().unwrap().trim().parse().unwrap();

            player = db::db::load_player(index as u8 - 1).unwrap();
        }
    }
    
    return player;
}