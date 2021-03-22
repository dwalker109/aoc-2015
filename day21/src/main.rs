use character::CharacterType;
use fight::run_matches;
use itertools::Itertools;

pub mod character;
pub mod equipment;
pub mod fight;

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
