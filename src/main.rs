mod creatures;
mod data;

use console::{Style, Term};
use creatures::{get_new_player, player, Spawner};
use data::db;
use rand::{rng, Rng};
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term: Term = Term::stdout();
    let mut player: player::Player;

    term.clear_screen()?;
    term.write_line("Hello Sir, Welcome to Omar's Adventure Game!")?;
    thread::sleep(Duration::from_millis(1000));

    term.clear_screen()?;
    let players = data::db::db::get_players();

    if players.is_err() {
        // If player doesnt exists, ask for his name and initialize new player

        player = get_new_player()?;
    } else {
        // If player exists ask to create new one or not
        term.write_line("Would you like to create new save? (y/n)")?;

        let answer = term.read_char().unwrap();

        term.clear_screen()?;

        if answer == 'y' {
            player = get_new_player()?;
            player.id = players.unwrap().len() as u8;
        } else {
            term.write_line("Select save: ")?;

            for (_, player) in players.unwrap().iter().enumerate() {
                term.write_line(&format!("{}. {}", player.0 + 1, player.1))?;
            }

            let index: usize = term.read_line().unwrap().trim().parse().unwrap();

            player = db::db::load_player(index as u8 - 1)?;
        }
    }

    term.clear_screen()?;
    term.write_line(&format!(
        "Hello {}!",
        Style::new().cyan().bold().apply_to(&player.name)
    ))?;

    term.write_line("For help type '='")?;

    thread::sleep(Duration::from_millis(1000));

    let mut spawner = Spawner { mobs: Vec::new() };

    loop {
        if spawner.mobs.len() == 0 {
            let count: u8 = rng().random_range(1..=5);
            spawner.spawn_mob(&player, count);
        }

        let key: char = term.read_char().unwrap();
        term.clear_screen()?;

        match key {
            'h' => {
                // Player heal and skip turn
                player.health += 10;
                term.write_line(&format!("You healed and skipped your turn"))?;

                thread::sleep(Duration::from_millis(100));

                println!(
                    "Your health is: {}",
                    term.style().red().apply_to(player.health)
                );

                let random_mob_index = rng().random_range(0..=spawner.mobs.len() - 1);

                // Mob attack Player
                let mut mob = spawner.mobs[random_mob_index].clone();

                mob.attack(&mut player);

                thread::sleep(Duration::from_millis(500));
                println!("{} Attacked you!", mob.name);
                println!("Your health is: {}", player.health)
            }

            'a' => {
                // Player attack
                println!("Which mob would you like to attack?");

                let index: char = term.read_char().unwrap();

                // Validate input

                let index = index.to_digit(10);

                let index: usize = match index {
                    Some(_) => index.unwrap() as usize - 1,
                    None => {
                        term.write_line("Invalid index")?;
                        continue;
                    }
                };

                if index > 4 {
                    term.write_line("Invalid index")?;
                    continue;
                } else if index > spawner.mobs.len() - 1 {
                    term.write_line("There are no mobs with that index")?;
                    continue;
                }

                // Get mob
                let mob: &mut creatures::mob::Mob = &mut spawner.mobs[index].clone();

                // Attack mob
                let is_crit: bool = player.attack(mob);
                if is_crit {
                    term.write_line(&format!("You crited {}!", mob.name))?;
                } else {
                    term.write_line(&format!("You attacked {}!", mob.name))?;
                }

                thread::sleep(Duration::from_millis(100));

                if mob.health <= 0 {
                    term.write_line(&format!("You killed {}!", mob.name))?;
                    spawner.mobs.remove(index);

                    println!("{} Died", mob.name);
                } else {
                    println!(
                        "{} health is: {}",
                        mob.name,
                        term.style().red().apply_to(mob.health)
                    );

                    // Mob attack Player
                    mob.attack(&mut player);

                    println!(
                        "{} Attacked you back!, your health now is: {}",
                        mob.name,
                        term.style().red().apply_to(player.health)
                    );

                    spawner.mobs[index] = mob.clone();
                }

                let _ = mob;
            }

            'm' => {
                // Display mobs
                let mut index: u8 = 1;

                for mob in &spawner.mobs {
                    println!(
                        "{}: {} health is: {}",
                        index,
                        mob.name,
                        term.style().red().apply_to(mob.health)
                    );
                    index += 1;
                }

                let _ = index;
            }

            's' => {
                // Status
                println!("Health: {}", term.style().red().apply_to(player.health));
                println!("Damage: {}", term.style().red().apply_to(player.damage));
                println!("Level: {}", term.style().red().apply_to(player.level));
                println!("Xp: {}", term.style().red().apply_to(player.xp));
                println!(
                    "Required xp: {}",
                    term.style().red().apply_to(player.required_xp())
                );
            }

            '=' => {
                // Help menu
                term.write_line("h: Heal")?;
                term.write_line("a: Attack <mob_index>")?;
                term.write_line("s: Status")?;
                term.write_line("m: Display Mobs")?;
                term.write_line("q: Quit")?;
            }

            'q' => {
                term.write_line("Goodbye!")?;
                break;
            }
            _ => {}
        }
    }

    thread::sleep(Duration::from_millis(500));

    db::db::save_player(&player)?;

    Ok(())
}
