use crate::{
    battle::Battle,
    characters::{Boss, Player},
};

use super::{prep_cast, Effect, EffectState, Spell};

#[derive(Debug, Clone)]
pub struct Recharge;
impl Spell for Recharge {
    fn cast(&self, battle: &mut Battle) -> Option<u32> {
        let mut effect = Box::new(RechargeEffect {
            duration: 5,
            mpt: 101,
            state: EffectState::New,
        });
        match prep_cast(battle, self.cost(), Some(effect.id())) {
            true => {
                effect.init(&mut battle.player, &mut battle.boss);
                battle.active_effects.push(effect);
                Some(self.cost())
            }
            false => None,
        }
    }

    fn cost(&self) -> u32 {
        229
    }
}

#[derive(Debug, Clone)]
struct RechargeEffect {
    duration: u32,
    mpt: u32,
    state: EffectState,
}
impl Effect for RechargeEffect {
    fn id(&self) -> &str {
        "recharge_effect"
    }

    fn init(&mut self, _player: &mut Player, _boss: &mut Boss) {
        self.state = EffectState::Running(self.duration);
    }

    fn tick(&mut self, player: &mut Player, _boss: &mut Boss) {
        player.mana += self.mpt;
        self.state = EffectState::Running(self.timer() - 1);

        if self.timer() == 0 {
            self.state = EffectState::Clear;
        }
    }

    fn state(&self) -> &EffectState {
        &self.state
    }
}
