use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> isize {
    let dat = std::fs::read_to_string(path).unwrap();

    let floor = dat.split("").fold(0, |acc, elem| match elem {
        "(" => return acc + 1,
        ")" => return acc - 1,
        _ => acc,
    });

    floor
}

fn part2(path: &str) -> usize {
    let dat = std::fs::read_to_string(path).unwrap();

    let pos = dat
        .split("")
        .enumerate()
        .fold_while(0 as isize, |mut acc, (pos, elem)| {
            match elem {
                "(" => acc += 1,
                ")" => acc -= 1,
                _ => (),
            }

            if acc == -1 {
                return Done(pos as isize);
            } else {
                Continue(acc)
            }
        })
        .into_inner();

    pos as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input"), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input"), 1771);
    }
}
