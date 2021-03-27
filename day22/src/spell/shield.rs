use crate::{
    battle::Battle,
    characters::{Boss, Player},
};

use super::{prep_cast, Effect, EffectState, Spell};

#[derive(Debug, Clone)]
pub struct Shield;
impl Spell for Shield {
    fn cast(&self, battle: &mut Battle) -> Option<u32> {
        let mut effect = Box::new(ShieldEffect {
            duration: 6,
            arm: 7,
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
        113
    }
}

#[derive(Debug, Clone)]
struct ShieldEffect {
    duration: u32,
    arm: u32,
    state: EffectState,
}
impl Effect for ShieldEffect {
    fn id(&self) -> &str {
        "shield_effect"
    }

    fn init(&mut self, _player: &mut Player, _boss: &mut Boss) {
        self.state = EffectState::Running(self.duration);
    }

    fn tick(&mut self, player: &mut Player, _boss: &mut Boss) {
        player.def = self.arm;
        self.state = EffectState::Running(self.timer() - 1);

        if self.timer() == 0 {
            player.def = 0;
            self.state = EffectState::Clear;
        }
    }

    fn state(&self) -> &EffectState {
        &self.state
    }
}
