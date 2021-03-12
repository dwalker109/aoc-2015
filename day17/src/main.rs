use itertools::Itertools;
use std::fs;

fn main() {
    let p1 = die_hard_3("./input", 150, false);
    let p2 = die_hard_3("./input", 150, true);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn die_hard_3(path: &str, target: u32, stop_at_min: bool) -> u32 {
    let data = fs::read_to_string(path).unwrap();
    let containers = data.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>();

    let mut tally = 0;
    for i in 1..containers.len() {
        for combo in containers.iter().combinations(i) {
            if combo.iter().fold(0, |acc, x| acc + *x) == target {
                tally += 1
            }
        }
        if stop_at_min && tally > 0 {
            break;
        }
    }

    tally
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(die_hard_3("./input_test", 25, false), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(die_hard_3("./input_test", 25, true), 3);
    }
}
