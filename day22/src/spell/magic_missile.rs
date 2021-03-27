use crate::battle::Battle;

use super::{prep_cast, Spell};

#[derive(Debug, Clone)]
pub struct MagicMissile;
impl Spell for MagicMissile {
    fn cast(&self, battle: &mut Battle) -> Option<u32> {
        match prep_cast(battle, self.cost(), None) {
            true => {
                battle.boss.guard(4);
                Some(self.cost())
            }
            false => None,
        }
    }

    fn cost(&self) -> u32 {
        53
    }
}
