use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::hash::BuildHasherDefault;

fn main() {
    let p1 = part1(29000000);

    println!("Part 1: {}", p1);
}

fn part1(target: usize) -> usize {
    let target = target / 10;
    let mut houses = FxHashMap::with_capacity_and_hasher(target, BuildHasherDefault::default());

    for elf in 1..=target {
        for house in (1..=target).skip(elf - 1).step_by(elf) {
            *houses.entry(house).or_insert(0) += elf;
        }
    }

    *houses
        .iter()
        .filter(|(_, &presents)| presents >= target)
        .sorted()
        .next()
        .unwrap()
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(100), 6);
    }
}
