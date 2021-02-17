use std::{cell::RefCell, collections::HashMap, fs};

use regex::Regex;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input", p1);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> u32 {
    let instructions = get_instructions(path);

    wire_up(instructions)
}

fn part2(path: &str, b_override: u32) -> u32 {
    let instructions = get_instructions(path)
        .into_iter()
        .map(|instruction| match instruction {
            Gate::Pass(_, wire) if wire == "b" => {
                Gate::Pass(format!("{}", b_override), "b".to_string())
            }
            _ => instruction,
        })
        .collect();

    wire_up(instructions)
}

enum Gate {
    Pass(String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u32, String),
    RShift(String, u32, String),
    Not(String, String),
}

fn get_instructions(path: &str) -> Vec<Gate> {
    let data = fs::read_to_string(path).expect("Could not read input");
    let opstring_regexp = Regex::new(r"([A-Z]+)").unwrap();
    let mut instructions: Vec<Gate> = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        match opstring_regexp.captures(line) {
            Some(captures) => match &captures[1] {
                "AND" => instructions.push(Gate::And(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                )),
                "OR" => instructions.push(Gate::Or(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                )),
                "LSHIFT" => instructions.push(Gate::LShift(
                    parts[0].to_string(),
                    parts[2].parse().unwrap(),
                    parts[4].to_string(),
                )),
                "RSHIFT" => instructions.push(Gate::RShift(
                    parts[0].to_string(),
                    parts[2].parse().unwrap(),
                    parts[4].to_string(),
                )),
                "NOT" => instructions.push(Gate::Not(parts[1].to_string(), parts[3].to_string())),
                _ => (),
            },
            None => instructions.push(Gate::Pass(parts[0].to_string(), parts[2].to_string())),
        };
    }

    instructions
}

fn wire_up(instructions: Vec<Gate>) -> u32 {
    let wire_map: RefCell<HashMap<&String, u32>> = RefCell::new(HashMap::new());

    let lookup_or_value = |signal: &&String| match signal.parse() {
        Ok(n) => return Some(n),
        Err(_) => match wire_map.borrow().get(signal) {
            Some(n) => return Some(*n),
            None => return None,
        },
    };

    while !wire_map.borrow().contains_key(&String::from("a")) {
        let mut completed: Vec<usize> = Vec::new();

        for (i, instruction) in instructions.iter().enumerate() {
            if completed.contains(&i) {
                continue;
            }

            let (k, v) = match instruction {
                Gate::Pass(signal, wire) => match lookup_or_value(&signal) {
                    Some(val) => (wire, val),
                    None => {
                        continue;
                    }
                },
                Gate::And(left, right, wire) => {
                    let val_l = lookup_or_value(&left);
                    let val_r = lookup_or_value(&right);
                    if val_l.is_none() || val_r.is_none() {
                        continue;
                    }
                    (wire, val_l.unwrap() & val_r.unwrap())
                }
                Gate::Or(left, right, wire) => {
                    let val_l = lookup_or_value(&left);
                    let val_r = lookup_or_value(&right);
                    if val_l.is_none() || val_r.is_none() {
                        continue;
                    }
                    (wire, val_l.unwrap() | val_r.unwrap())
                }
                Gate::LShift(source, amount, wire) => match lookup_or_value(&source) {
                    Some(val) => (wire, val << *amount),
                    None => {
                        continue;
                    }
                },
                Gate::RShift(source, amount, wire) => match lookup_or_value(&source) {
                    Some(val) => (wire, val >> *amount),
                    None => {
                        continue;
                    }
                },
                Gate::Not(source, wire) => match lookup_or_value(&source) {
                    Some(val) => (wire, !val),
                    None => {
                        continue;
                    }
                },
            };

            wire_map.borrow_mut().insert(k, v);
            completed.push(i);
        }
    }

    *wire_map.into_inner().get(&String::from("a")).unwrap()
}

#[test]
fn test_part1() {
    let p1 = part1("./input");
    assert_eq!(p1, 46065)
}

#[test]
fn test_part2() {
    let p2 = part2("./input", 46065);
    assert_eq!(p2, 14134)
}
