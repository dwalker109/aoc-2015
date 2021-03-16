use itertools::Itertools;
use std::fs;

fn main() {
    let p1 = part1("./input");

    println!("Part 1: {}", p1);
}

fn part1(path: &str) -> usize {
    let data = fs::read_to_string(path).unwrap();
    let (replacements, molecule) = parse_input(&data);

    let mut new_molecules = Vec::new();
    for (from, to) in replacements {
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
        assert_eq!(part1("./input_test"), 7)
    }
}
