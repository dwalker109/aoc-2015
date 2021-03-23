use crate::{
    battle::Battle,
    characters::{Boss, Player},
};

use super::{prep_cast, Effect, EffectState, Spell};

#[derive(Debug, Clone)]
pub struct Poison;
impl Spell for Poison {
    fn cast(&self, battle: &mut Battle) -> Option<u32> {
        let mut effect = Box::new(PoisonEffect {
            duration: 6,
            dpt: 3,
            state: EffectState::New,
        });
        match prep_cast(
            &mut battle.player,
            self.cost(),
            Some(effect.id()),
            &battle.active_effects,
        ) {
            true => {
                effect.init(&mut battle.player, &mut battle.boss);
                battle.active_effects.push(effect);
                Some(self.cost())
            }
            false => None,
        }
    }

    fn cost(&self) -> u32 {
        173
    }
}

#[derive(Debug, Clone)]
struct PoisonEffect {
    duration: u32,
    dpt: u32,
    state: EffectState,
}
impl Effect for PoisonEffect {
    fn id(&self) -> &str {
        "poison_effect"
    }

    fn init(&mut self, _player: &mut Player, _bosss: &mut Boss) {
        self.state = EffectState::Running(self.duration);
    }

    fn tick(&mut self, _player: &mut Player, boss: &mut Boss) {
        boss.guard(self.dpt);
        self.state = EffectState::Running(self.timer() - 1);

        if self.timer() == 0 {
            self.state = EffectState::Clear;
        }
    }

    fn state(&self) -> &EffectState {
        &self.state
    }
}
