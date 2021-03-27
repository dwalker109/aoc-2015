use crate::battle::Battle;

use super::{prep_cast, Spell};

#[derive(Debug, Clone)]
pub struct Drain;
impl Spell for Drain {
    fn cast(&self, battle: &mut Battle) -> Option<u32> {
        match prep_cast(battle, self.cost(), None) {
            true => {
                battle.player.hp += 2;
                battle.boss.guard(2);
                Some(self.cost())
            }
            false => None,
        }
    }

    fn cost(&self) -> u32 {
        73
    }
}
