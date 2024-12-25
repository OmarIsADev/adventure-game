use rusqlite::Connection;

pub fn init_db() -> Connection {
    let db: Connection = Connection::open("adventure_game_data.db").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS player (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            health INTEGER NOT NULL,
            damage INTEGER NOT NULL,
            crit INTEGER NOT NULL,
            level INTEGER NOT NULL,
            xp INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();

    db
}

pub mod db {
    use super::init_db;
    use crate::player::Player;
    use rusqlite::{params, Connection};

    pub fn save_player(player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let db: Connection = init_db();

        db.execute(
            "INSERT INTO player (id, name, health, damage, crit, level, xp) VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            health = excluded.health,
            damage = excluded.damage,
            crit = excluded.crit,
            level = excluded.level,
            xp = excluded.xp",
            params![
                player.id,
                player.name,
                player.health,
                player.damage,
                player.crit,
                player.level,
                player.xp
            ],
        )?;

        Ok(())
    }

    pub fn get_players() -> Result<Vec<(i32, String)>, Box<dyn std::error::Error>> {
        let db: Connection = init_db();
    
        let mut binding = db.prepare("SELECT id, name FROM player")?;
        let stmt = binding.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((id, name))
        })?;

        let mut players: Vec<(i32, String)> = Vec::new();
    
        for player in stmt {
            match player {
                Ok(player) => players.push(player),
                Err(e) => println!("Error: {}", e),
            }
        }
    
        if players.is_empty() {
            return Err("No players found".into());
        }

        Ok(players)
    }

    pub fn load_player(index: u8) -> Result<Player, Box<dyn std::error::Error>> {
        let db: Connection = init_db();

        let player: Player = db.query_row(
            "SELECT id, name, health, damage, crit, level, xp FROM player WHERE id = (?)",
            [index],
            |row| {
                Ok(Player {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    health: row.get(2)?,
                    damage: row.get(3)?,
                    crit: row.get(4)?,
                    level: row.get(5)?,
                    xp: row.get(6)?,
                })
            },
        )?;

        Ok(player)
    }
}
