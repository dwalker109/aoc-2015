use crate::battle::Battle;

use super::{prep_cast, Spell};

#[derive(Debug, Clone)]
pub struct MagicMissile;
impl Spell for MagicMissile {
    fn cast(&self, battle: &mut Battle) -> Option<u32> {
        match prep_cast(
            &mut battle.player,
            self.cost(),
            None,
            &battle.active_effects,
        ) {
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
