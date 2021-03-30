use std::{fs::read_to_string, slice::Iter};

fn main() {
    println!("Hello, world!");
}

fn part1(path: &str) -> u32 {
    let input = read_to_string(path).unwrap();
    let inst = parse_input(&input);
}

enum Inst {
    Hlf(char),
    TPl(char),
    Inc(char),
    Jmp(u32),
    Jie(char, u32),
    Jio(char, u32),
}

fn parse_input(input: &str) -> impl Iterator<Item = Inst> + '_ {
    input.lines().map(|l| {
        let p = l
            .replace(",", "")
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        let ext_reg = || -> char { p[1].parse().unwrap() };
        let ext_off = || -> u32 { p[1].parse().unwrap() };
        let ext_reg_off = || -> (char, u32) { (p[1].parse().unwrap(), p[2].parse().unwrap()) };

        match p[0] {
            "hlf" => Inst::Hlf(ext_reg()),
            "tpl" => Inst::TPl(ext_reg()),
            "inc" => Inst::Inc(ext_reg()),
            "jmp" => Inst::Jmp(ext_off()),
            "jie" => {
                let (r, o) = ext_reg_off();
                Inst::Jie(r, o)
            }
            "jio" => {
                let (r, o) = ext_reg_off();
                Inst::Jio(r, o)
            }
            _ => panic!("welp"),
        }
    })
}
