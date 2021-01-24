use std::{fs::read_to_string};

fn main() {
    let p1 = part1("./input");

    println!("Part 1: {}", p1);
}

fn part1(path: &str) -> usize {
    let input = read_to_string(path);
    let data = input.unwrap();
    let lines = data.lines();

    let processed = lines.filter(|&line| {
        let total = line.trim().chars().fold(0, |mut acc, c| {
            match ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                true => acc += 1,
                false => (),
            };
            
            acc
        });

        total >= 3
    })
    .filter(|&line| {
        let mut last = '_';
        for c in line.trim().chars() {
            if c == last {
                return true;
            } else {
                last = c;
                continue;
            }
        }

        false
    })
    .filter(|&line| {
        let res = ["ab", "cd", "pq", "xy"].iter().try_for_each(|&banned| {
            match line.trim().contains(banned) {
                true => Err(()),
                false => Ok(()),
            }
        });

        res.is_ok()
    });

    processed.count()
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input_test"), 2);
}
