#![feature(drain_filter)]

use std::cmp::min;

use battle::{Battle, Outcome};
use characters::{Boss, Player};
use spell::spellbook;

fn main() {
    let p1 = part1();

    println!("Part 1: {}", p1);
}

fn part1() -> u32 {
    fight()
}

fn fight() -> u32 {
    let mut result = u32::MAX;

    let battle = Battle {
        player: Player {
            hp: 50,
            def: 0,
            mana: 500,
        },
        boss: Boss { hp: 51, dmg: 9 },
        mana_spent: 0,
        outcome: battle::Outcome::Unknown,
        active_effects: Vec::new(),
    };

    fn round_perm(battle: Battle, result: &mut u32) {
        for spell in spellbook() {
            let mut battle = battle.clone();

            if battle.mana_spent > *result {
                continue;
            }

            battle.player_turn(spell);

            if let Outcome::PlayerWin(mana) = battle.outcome {
                *result = min(*result, mana);
                continue;
            }

            battle.boss_turn();

            match battle.outcome {
                Outcome::Unknown => round_perm(battle.clone(), result),
                Outcome::PlayerWin(mana) => {
                    *result = min(*result, mana);
                    continue;
                }
                Outcome::BossWin => {
                    continue;
                }
            }
        }
    }

    round_perm(battle, &mut result);

    result
}

#[cfg(test)]
mod test {
    use crate::spell::{
        drain::Drain, magic_missile::MagicMissile, poison::Poison, recharge::Recharge,
        shield::Shield,
    };

    use super::*;

    #[test]
    fn test_part1() {
        let mut battle = Battle {
            player: Player {
                hp: 10,
                def: 0,
                mana: 250,
            },
            boss: Boss { hp: 14, dmg: 8 },
            active_effects: Vec::new(),
            mana_spent: 0,
            outcome: Outcome::Unknown,
        };

        battle.player_turn(Box::new(Recharge));
        battle.boss_turn();
        battle.player_turn(Box::new(Shield));
        battle.boss_turn();
        battle.player_turn(Box::new(Drain));
        battle.boss_turn();
        battle.player_turn(Box::new(Poison));
        battle.boss_turn();
        battle.player_turn(Box::new(MagicMissile));
        battle.boss_turn();
        assert!(matches!(battle.outcome, Outcome::PlayerWin(_)));
        assert!(battle.boss.hp == 0);
        assert!(battle.player.hp == 1);
    }

    #[test]
    fn test_part2() {}
}

pub mod battle;
pub mod characters;
pub mod spell;
