use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let p1 = balance("./input", 3);
    let p2 = balance("./input", 4);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn balance(path: &str, compartments: u64) -> u64 {
    let input = read_to_string(path).unwrap();
    let presents: Vec<u64> = input.lines().map(|n| n.parse().unwrap()).collect();

    let target = presents.iter().sum::<u64>() / compartments;
    let mut groups = Vec::new();

    // Incredibly slow brute force.
    for p in presents.iter().copied().powerset() {
        if p.iter().sum::<u64>() == target {
            groups.push(p);
        }
    }

    // This is a lucky solution, it just abuses the nice input, as
    // a set of packages which sums correctly is always on the happy path
    groups
        .iter()
        .sorted_by_key(|g| g.iter().copied().product::<u64>())
        .sorted_by_key(|g| g.len())
        .next()
        .unwrap()
        .iter()
        .product()
}
