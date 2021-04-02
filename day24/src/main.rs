use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let p1 = balance("./input", 3);
    let p2 = balance("./input", 4);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

// This function works with my input but isn't a general solution.
// It just finds the first (shortest) combination of presents which hits the
// target - and it just happens to work because the input is friendly!
fn balance(path: &str, compartments: u64) -> u64 {
    let input = read_to_string(path).unwrap();
    let presents: Vec<u64> = input.lines().map(|n| n.parse().unwrap()).collect();
    let target = presents.iter().sum::<u64>() / compartments;

    for p in presents.iter().copied().powerset() {
        if p.iter().sum::<u64>() == target {
            return p.iter().product();
        }
    }

    panic!("erm...");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(balance("./input", 3), 10723906903);
    }

    #[test]
    fn test_part2() {
        assert_eq!(balance("./input", 4), 74850409);
    }
}
