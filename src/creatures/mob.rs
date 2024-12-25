use rand::{rng, Rng};

use super::player::Player;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Mob {
    pub name: String,
    pub health: i32,
    pub damage: u32,
    pub crit: u8,
    pub level: u8,
}

impl Mob {
    pub fn attack(&mut self, target: &mut Player) -> bool {
        let chance: u8 = rng().random_range(1..=100);
        let is_crit = chance <= self.crit;
        let mut damage: u32 = self.damage;

        if is_crit{
            damage = damage * 2;
        } 
        
        target.health -= damage as i16;
        
    
        println!("{} hit {} for {} damage", self.name, target.name, damage);
        println!("{} has {} health left", target.name, target.health);

        is_crit
    }

    pub fn decrease_health(&mut self, damage: u32, crit_chance: u8) -> bool {
        let chance: u8 = rng().random_range(1..=100);
        let mut damage: u32 = damage;
        let is_crit: bool = chance <= crit_chance;

        if is_crit {
            damage = damage * 2;
        }

        self.health -= damage as i32;

        is_crit
    }
}