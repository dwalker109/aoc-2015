use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::hash::BuildHasherDefault;

fn main() {
    let p1 = part1(29000000);
    let p2 = part2(29000000, 50);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(target: usize) -> usize {
    let size = target / 10;
    let mut houses = FxHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default());

    for elf in 1..=size {
        for house in (1..=size).skip(elf - 1).step_by(elf) {
            let h_entry = houses.entry(house).or_insert(0);
            *h_entry += elf * 10;

            if *h_entry >= target {
                break;
            }
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

fn part2(target: usize, stop_after: u32) -> usize {
    let size = target / 10;
    let mut houses = FxHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default());
    let mut log = FxHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default());

    for elf in 1..=size {
        for house in (1..=size).skip(elf - 1).step_by(elf) {
            let h_entry = houses.entry(house).or_insert(0);
            *h_entry += elf * 11;

            let l_entry = log.entry(elf).or_insert(0);
            *l_entry += 1;

            if *h_entry >= target || *l_entry >= stop_after {
                break;
            }
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(120, 2), 8);
    }
}
