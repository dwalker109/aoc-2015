use day21::{
    character::{Character, CharacterType},
    fight::{Item, Outcome},
};
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;

fn main() {
    let p1 = part1(100, 103, 9, 2);
    let p2 = part2(100, 103, 9, 2);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(player_hp: u32, boss_hp: u32, boss_dmg: u32, boss_def: u32) -> u32 {
    let outcomes = run_matches(player_hp, boss_hp, boss_dmg, boss_def);

    outcomes
        .iter()
        .filter(|o| o.winner == CharacterType::Player)
        .sorted_by_key(|o| o.gold_spent)
        .next()
        .unwrap()
        .gold_spent
}

fn part2(player_hp: u32, boss_hp: u32, boss_dmg: u32, boss_def: u32) -> u32 {
    let outcomes = run_matches(player_hp, boss_hp, boss_dmg, boss_def);

    outcomes
        .iter()
        .filter(|o| o.winner == CharacterType::Boss)
        .sorted_by_key(|o| o.gold_spent)
        .last()
        .unwrap()
        .gold_spent
}

fn run_matches(player_hp: u32, boss_hp: u32, boss_dmg: u32, boss_def: u32) -> Vec<Outcome> {
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

lazy_static! {
    static ref WEAPONS: [Item; 5] = [
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];
}

lazy_static! {
    static ref ARMOR: [Item; 6] = [
        Item::new("None", 0, 0, 0),
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];
}

lazy_static! {
    static ref RINGS: [Item; 8] = [
        Item::new("None", 0, 0, 0),
        Item::new("None", 0, 0, 0),
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(100, 103, 9, 2), 121);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(100, 103, 9, 2), 201);
    }
}
