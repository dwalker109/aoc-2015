use regex::Regex;
use std::{collections::HashSet, fs};

fn main() {
    let p1 = part1("./input");

    println!("Part 1: {}", p1);
}

enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(PartialEq, Eq, Hash)]
struct Coordinates(u32, u32);

struct Instruction(Op, Coordinates, Coordinates);

impl Instruction {
    fn expand_coordinates(&self) -> Vec<Coordinates> {
        let Coordinates(lx, ly) = self.1;
        let Coordinates(rx, ry) = self.2;

        let mut list: Vec<Coordinates> = Vec::new();

        for row in lx..=rx {
            for col in ly..=ry {
                list.push(Coordinates(row, col));
            }
        }

        list
    }
}

fn part1(path: &str) -> u32 {
    let instructions = get_instructions(path);
    let mut lit: HashSet<Coordinates> = HashSet::new();

    for instruction in instructions {
        let expanded = instruction.expand_coordinates();
        for coordinate in expanded {
            match instruction.0 {
                Op::TurnOn => {
                    lit.insert(coordinate);
                }
                Op::TurnOff => {
                    lit.remove(&coordinate);
                }
                Op::Toggle => {
                    if lit.contains(&coordinate) {
                        lit.remove(&coordinate);
                    } else {
                        lit.insert(coordinate);
                    }
                }
            }
        }
    }

    lit.len() as u32
}

fn get_instructions(path: &str) -> Vec<Instruction> {
    let data = fs::read_to_string(path).expect("Could not read input");
    let extract_regex = Regex::new(
        r"(?P<op>turn on|turn off|toggle)\s(?P<lx>\d*),(?P<ly>\d*)\s\w*\s(?P<rx>\d*),(?P<ry>\d*)",
    )
    .unwrap();

    let instructions = data
        .lines()
        .flat_map(|line| {
            extract_regex.captures_iter(line).map(|cap| {
                let op = match &cap["op"] {
                    "turn on" => Op::TurnOn,
                    "turn off" => Op::TurnOff,
                    "toggle" => Op::Toggle,
                    _ => {
                        panic!("Zoinks!")
                    }
                };
                let x = Coordinates(cap["lx"].parse().unwrap(), cap["ly"].parse().unwrap());
                let y = Coordinates(cap["rx"].parse().unwrap(), cap["ry"].parse().unwrap());

                Instruction(op, x, y)
            })
        })
        .collect::<Vec<_>>();

    instructions
}

#[test]
fn test_part1() {
    let result = part1("./input_test");
    assert_eq!(result, 1626);
}
