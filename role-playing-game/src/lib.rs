// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

// https://exercism.org/tracks/rust/exercises/role-playing-game/solutions/koushik-ms
//  Liked use of new method and used match in revive and cast spell
impl Player {
    pub fn revive(&self) -> Option<Player> {
        let level = self.level;
        if self.health == 0 {
            let mana = if self.level >= 10 { Some(100) } else { None };
            let player = Player {
                health: 100,
                mana,
                level,
            };
            return Some(player);
        } else {
            return None;
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let damage = 2 * mana_cost;
        if let Some(mana) = self.mana {
            // If the player has a mana pool but insufficient mana,
            // the method should not affect the pool,
            // but instead return 0
            if mana < damage {
                return 0;
            }
            // Otherwise, the mana_cost should be deducted from the
            // Player's mana pool and the appropriate amount of damage
            // should be returned.
            let mana = mana - mana_cost;
            self.mana = Some(mana);
            return damage;
        } else {
            // If the player does not have access to a mana pool,
            // attempting to cast the spell must decrease their
            // health by the mana cost of the spell.
            // The damage returned must be 0.
            if mana_cost > self.health {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }
            return 0;
        }
    }
}
