use crate::character::{Character, CharacterType};
use crate::equipment::*;
use itertools::Itertools;
use rayon::prelude::*;

fn get_builds(hp: u32) -> Vec<Character> {
    let mut builds = Vec::new();

    let ring_options = RINGS
        .iter()
        .tuple_combinations()
        .collect::<Vec<(&Item, &Item)>>();

    for wpn in WEAPONS.iter() {
        for arm in ARMOR.iter() {
            for (r1, r2) in ring_options.iter() {
                builds.push(Character::new(
                    CharacterType::Player,
                    hp,
                    0,
                    0,
                    Some(wpn),
                    Some(arm),
                    Some([r1, r2]),
                ))
            }
        }
    }

    builds
}

fn run_game<'a>(player: &'a mut Character, boss: &'a mut Character) -> Outcome {
    let gold_spent = player.gold_spent();
    let (p1, p2) = (player, boss);

    let winner = loop {
        (p1).attack(p2);
        if p2.is_defeated() {
            break p1;
        }
        std::mem::swap(p1, p2);
    };

    Outcome {
        winner: match winner.is_player() {
            true => CharacterType::Player,
            false => CharacterType::Boss,
        },
        gold_spent,
    }
}

pub fn run_matches(player_hp: u32, boss_hp: u32, boss_dmg: u32, boss_def: u32) -> Vec<Outcome> {
    let mut builds = get_builds(player_hp);
    let boss = Character::new(
        CharacterType::Boss,
        boss_hp,
        boss_dmg,
        boss_def,
        None,
        None,
        None,
    );

    builds
        .par_iter_mut()
        .map(|c| run_game(c, &mut boss.clone()))
        .collect()
}

#[derive(Debug)]
pub struct Outcome {
    pub winner: CharacterType,
    pub gold_spent: u32,
}
