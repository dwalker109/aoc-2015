use drain::Drain;
use dyn_clone::{clone_trait_object, DynClone};
use magic_missile::MagicMissile;
use poison::Poison;
use recharge::Recharge;
use shield::Shield;

use crate::{
    battle::Battle,
    characters::{Boss, Player},
};

pub mod drain;
pub mod magic_missile;
pub mod poison;
pub mod recharge;
pub mod shield;

pub trait Spell: DynClone {
    fn cast(&self, battle: &mut Battle) -> Option<u32>;
    fn cost(&self) -> u32;
}

fn prep_cast(
    player: &mut Player,
    cost: u32,
    effect_id: Option<&str>,
    active_effects: &[Box<dyn Effect>],
) -> bool {
    if let Some(effect_id) = effect_id {
        if active_effects.iter().any(|e| (e.id() == effect_id)) {
            return false;
        }
    }

    if cost > player.mana {
        false
    } else {
        player.mana -= cost;
        true
    }
}

clone_trait_object!(Spell);

pub trait Effect: DynClone {
    fn id(&self) -> &str;
    fn init(&mut self, player: &mut Player, boss: &mut Boss);
    fn tick(&mut self, player: &mut Player, boss: &mut Boss);
    fn state(&self) -> &EffectState;
    fn timer(&self) -> u32 {
        match self.state() {
            EffectState::Running(timer) => *timer,
            _ => 0,
        }
    }
}

clone_trait_object!(Effect);

#[derive(Debug, Clone, Copy)]
pub enum EffectState {
    New,
    Running(u32),
    Done,
    Clear,
}

pub fn spellbook() -> Vec<Box<dyn Spell>> {
    vec![
        Box::new(MagicMissile),
        Box::new(Drain),
        Box::new(Shield),
        Box::new(Poison),
        Box::new(Recharge),
    ]
}
