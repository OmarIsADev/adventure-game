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
