//! Rust regexes can't do backreferences so this turned into a real iterator adapter crash course

use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn get_lines(path: &str) -> Vec<String> {
    let data = read_to_string(path).expect("Could not read input");
    let lines: Vec<String> = data.lines().map(|l| l.to_owned()).collect();

    lines
}

fn part1(path: &str) -> usize {
    let lines = get_lines(path);

    let processed = lines
        .iter()
        .filter(|&line| {
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

fn part2(path: &str) -> usize {
    let lines = get_lines(path);

    let processed = lines
        .iter()
        .filter(|&line| {
            let indexed_chars = line.chars().enumerate().collect::<Vec<_>>();
            let pair_windows = indexed_chars.windows(2);

            /* This HashMap is keyed by the pair, and the val contains the index of the prev
            second letter (to avoid overlaps) and total number of occurrences */
            let init: HashMap<(char, char), (usize, u32)> = HashMap::new();

            let result = pair_windows.fold(init, |mut acc, el| {
                let (a_index, a_char) = el[0];
                let (b_index, b_char) = el[1];

                match acc.get_mut(&(a_char, b_char)) {
                    Some(pair) => {
                        let (prev_b_index, count) = pair;
                        if !(a_index == *prev_b_index) {
                            *pair = (b_index, *count + 1);
                        }
                    }
                    None => {
                        acc.insert((a_char, b_char), (b_index, 1));
                    }
                }

                acc
            });

            result.values().filter(|(_, count)| *count >= 2).count() > 0
        })
        .filter(|&line| {
            let indexed_chars = line.chars().collect::<Vec<_>>();
            let triplet_windows = indexed_chars.windows(3);

            let result = triplet_windows.fold(0, |mut acc, el| {
                if el[0] == el[2] {
                    acc += 1;
                }

                acc
            });

            result > 0
        });

    let left = processed.collect::<Vec<_>>();

    left.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input"), 236);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input"), 51);
}
