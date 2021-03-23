use crate::{
    characters::{Boss, Player},
    spell::{Effect, EffectState, Spell},
};

#[derive(Debug, Clone)]
pub enum Outcome {
    Unknown,
    PlayerWin(u32),
    BossWin,
}

#[derive(Clone)]
pub struct Battle {
    pub player: Player,
    pub boss: Boss,
    pub active_effects: Vec<Box<dyn Effect>>,
    pub mana_spent: u32,
    pub outcome: Outcome,
}

impl Battle {
    pub fn player_turn(&mut self, spell: Box<dyn Spell>) {
        self.run_effects();
        self.clean_up_effects();

        if self.boss.hp == 0 {
            self.outcome = Outcome::PlayerWin(self.mana_spent);
            return;
        }

        match spell.cast(self) {
            Some(mana) => self.mana_spent += mana,
            None => self.outcome = Outcome::BossWin,
        };
    }

    pub fn boss_turn(&mut self) {
        self.run_effects();
        self.clean_up_effects();

        if self.boss.hp == 0 {
            self.outcome = Outcome::PlayerWin(self.mana_spent);
            return;
        }

        self.boss.attack(&mut self.player);

        if self.player.hp == 0 {
            self.outcome = Outcome::BossWin;
        }
    }

    pub fn run_effects(&mut self) {
        let effects = self.active_effects.iter_mut();
        for effect in effects {
            if let EffectState::Running(_) = effect.state() {
                effect.tick(&mut self.player, &mut self.boss)
            }
        }
    }

    fn clean_up_effects(&mut self) {
        self.active_effects
            .drain_filter(|effect| matches!(effect.state(), EffectState::Clear));
    }
}
