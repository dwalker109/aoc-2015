use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let p1 = compute("./input", 0, 0);
    let p2 = compute("./input", 1, 0);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn compute(path: &str, reg_a: u32, reg_b: u32) -> u32 {
    let input = read_to_string(path).unwrap();
    let inst = parse_input(&input);
    let mut registers: HashMap<char, u32> = vec![('a', reg_a), ('b', reg_b)].into_iter().collect();

    let mut n = 0;
    while let Some(i) = inst.get(n as usize) {
        match i {
            Inst::Hlf(r) => {
                let v = registers.get_mut(&r).unwrap();
                *v /= 2;
                n += 1;
            }
            Inst::TPl(r) => {
                let v = registers.get_mut(&r).unwrap();
                *v *= 3;
                n += 1;
            }
            Inst::Inc(r) => {
                let v = registers.get_mut(&r).unwrap();
                *v += 1;
                n += 1;
            }
            Inst::Jmp(o) => {
                n += *o as isize;
            }
            Inst::Jie(r, o) => {
                let v = registers.get(&r).unwrap();
                if *v % 2 == 0 {
                    n += *o as isize
                } else {
                    n += 1;
                }
            }
            Inst::Jio(r, o) => {
                let v = registers.get(&r).unwrap();
                if *v == 1 {
                    n += *o as isize
                } else {
                    n += 1;
                }
            }
        }
    }

    *registers.get(&'b').unwrap()
}

enum Inst {
    Hlf(char),
    TPl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

fn parse_input(input: &str) -> Vec<Inst> {
    input
        .lines()
        .map(|l| {
            let p = l.replace(",", "");
            let p = p.split_ascii_whitespace().collect::<Vec<_>>();

            let ext_reg = || -> char { p[1].parse().unwrap() };
            let ext_off = || -> i32 { p[1].parse().unwrap() };
            let ext_reg_off = || -> (char, i32) { (p[1].parse().unwrap(), p[2].parse().unwrap()) };

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
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(compute("./input", 0, 0), 255);
    }

    #[test]
    fn test_p2() {
        assert_eq!(compute("./input", 1, 0), 334);
    }
}
