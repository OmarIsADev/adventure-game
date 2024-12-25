use console::Term;
use mob::Mob;
use player::Player;
use rand::{rng, Rng};

pub mod mob;
pub mod player;

pub struct Spawner {
    pub mobs: Vec<Mob>,
}

impl Spawner {
    pub fn spawn_mob(&mut self, player: &Player, count: u8) {
        for _ in 0..count {
            let rand: u8 = rng().random_range(0..=4);
            let mob_level: u8 = rng().random_range(1..=player.level);
            let mob = match rand {
                1 => Mob {
                    name: String::from("Goblin"),
                    health: 15,
                    damage: 4,
                    crit: 20,
                    level: mob_level,
                },
                2 => Mob {
                    name: String::from("Skeleton"),
                    health: 10,
                    damage: 3,
                    crit: 20,
                    level: mob_level,
                },
                _ => Mob {
                    name: String::from("Zombie"),
                    health: 5,
                    damage: 2,
                    crit: 20,
                    level: mob_level,
                },
            };

            let _ = &self.mobs.push(mob);
        }
    }
}

pub fn get_new_player() -> Result<player::Player, Box<dyn std::error::Error>> {
    let term = Term::stdout();

    term.write_line("What should i call you?")?;
    let name: String = term.read_line()?;
    Ok(player::Player {
        id: 0,
        name: name.trim().to_string(),
        health: 1000,
        damage: 1,
        crit: 20,
        level: 1,
        xp: 0,
    })
}