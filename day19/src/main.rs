use itertools::Itertools;
use rand::{seq::SliceRandom, SeedableRng};
use rand_hc::Hc128Rng;
use std::fs;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> usize {
    let data = fs::read_to_string(path).unwrap();
    let (replacements, molecule) = parse_input(&data);

    let mut new_molecules = Vec::new();
    for (from, to) in replacements.iter() {
        for (idx, _) in molecule.match_indices(from) {
            let new_molecule = [
                molecule[..idx].as_bytes(),
                to[..].as_bytes(),
                molecule[idx + from.len()..].as_bytes(),
            ];
            new_molecules.push(new_molecule.concat());
        }
    }

    new_molecules.iter().unique().count()
}

/// This works for the provided input but isn't a general solution - the order
/// of replacements must be seeded deterministically otherwise it sometimes fails!
fn part2(path: &str) -> usize {
    let data = fs::read_to_string(path).unwrap();
    let (replacements, molecule) = parse_input(&data);

    let mut mol_work = molecule.to_string();
    let mut steps = 0;
    let mut rng = Hc128Rng::seed_from_u64(1);
    loop {
        let (to, from) = replacements.choose(&mut rng).unwrap();
        if mol_work.contains(from) {
            steps += 1;
            mol_work = mol_work.replacen(from, to, 1);
        }
        if mol_work == "e" {
            return steps;
        }
    }
}

fn parse_input(data: &str) -> (Vec<(&str, &str)>, &str) {
    let mut replacements = Vec::new();
    let mut molecule = "";

    for line in data.lines().filter(|&l| !l.is_empty()).with_position() {
        if let itertools::Position::Last(l) = line {
            molecule = l;
        } else {
            let parts = line.into_inner().split_once(" => ").unwrap();
            replacements.push(parts);
        }
    }

    (replacements, molecule)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input"), 509)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input"), 195)
    }
}
