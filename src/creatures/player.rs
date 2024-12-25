use rand::{rng, Rng};

use super::mob::Mob;

#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub health: i16,
    pub damage: u16,
    pub crit: u8,
    pub level: u8,
    pub xp: u16,
    // inventroy: Vec<Item>,
}

impl Player {
    pub fn attack(&mut self, target: &mut Mob) -> bool {
        let chance = rng().random_range(0..=100);
        let is_crit = chance <= self.crit;
        let mut damage = self.damage;

        if is_crit {
            damage *= 2
        }

        target.health -= damage as i32;

        if target.health <= 0 {
            self.calculate_xp(target.level as i8);
        }

        is_crit
    }

    fn calculate_xp(&mut self, level: i8) {
        self.xp += (level * level + -2 * level + 2) as u16;

        if self.xp >= self.required_xp() {
            self.level += 1;
        }
    }

    pub fn required_xp(&self) -> u16 {
        let level = self.level;

        return (level * level + 4 * level + 2) as u16;
    }
}
