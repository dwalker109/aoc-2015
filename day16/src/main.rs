use std::{collections::HashMap, fs};

use lazy_static::lazy_static;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(Debug)]
struct Sue<'a>(u32, HashMap<&'a str, u32>);

lazy_static! {
    static ref MFCSAM: HashMap<&'static str, u32> = {
        let mut mfcsam = HashMap::new();
        mfcsam.insert("children", 3);
        mfcsam.insert("cats", 7);
        mfcsam.insert("samoyeds", 2);
        mfcsam.insert("pomeranians", 3);
        mfcsam.insert("akitas", 0);
        mfcsam.insert("vizslas", 0);
        mfcsam.insert("goldfish", 5);
        mfcsam.insert("trees", 3);
        mfcsam.insert("cars", 2);
        mfcsam.insert("perfumes", 1);
        mfcsam
    };
}

fn part1(path: &str) -> u32 {
    let data = fs::read_to_string(path).unwrap();
    let sues = parse_input(&data);

    let sue = sues.iter().find(|Sue(_, memories)| {
        for (&fact, qty) in MFCSAM.iter() {
            if let Some(v) = memories.get(fact) {
                if v != qty {
                    return false;
                }
            }
        }
        true
    });

    sue.unwrap().0
}

fn part2(path: &str) -> u32 {
    let data = fs::read_to_string(path).unwrap();
    let sues = parse_input(&data);

    let sue = sues.iter().find(|Sue(_, memories)| {
        for (&fact, fact_qty) in MFCSAM.iter() {
            if let Some(recollection) = memories.get(fact) {
                match fact {
                    "cats" | "trees" => {
                        if recollection <= fact_qty {
                            return false;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if recollection >= fact_qty {
                            return false;
                        }
                    }
                    _ => {
                        if recollection != fact_qty {
                            return false;
                        }
                    }
                }
            }
        }
        true
    });

    sue.unwrap().0
}

fn parse_input<'a>(data: &'a str) -> Vec<Sue<'a>> {
    data.lines()
        .map(|s| {
            let parts = s.splitn(2, ": ").collect::<Vec<_>>();
            let sue_num = parts[0]
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let mut hm: HashMap<&str, u32> = HashMap::new();
            for memory in parts[1].split(", ") {
                let kv = memory.splitn(2, ": ").collect::<Vec<_>>();
                hm.insert(kv[0], kv[1].parse().unwrap());
            }
            Sue(sue_num, hm)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input"), 40)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input"), 241)
    }
}
