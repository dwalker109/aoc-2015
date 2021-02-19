use std::collections::HashSet;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> usize {
    let dat = std::fs::read_to_string(path).unwrap();

    let mut curr_house = (0, 0);
    let mut houses: HashSet<(isize, isize)> = HashSet::new();

    houses.insert(curr_house);
    dat.split_terminator("")
        .for_each(|dir| follow_dir(dir, &mut curr_house, &mut houses));

    houses.len()
}

fn part2(path: &str) -> usize {
    let dat = std::fs::read_to_string(path).unwrap();

    let mut santa_curr_house = (0, 0);
    let mut robosanta_curr_house = (0, 0);
    let mut houses: HashSet<(isize, isize)> = HashSet::new();

    houses.insert(santa_curr_house);
    houses.insert(robosanta_curr_house);

    dat.split_terminator("")
        .step_by(2)
        .for_each(|dir| follow_dir(dir, &mut santa_curr_house, &mut houses));

    dat.split_terminator("")
        .skip(1)
        .step_by(2)
        .for_each(|dir| follow_dir(dir, &mut robosanta_curr_house, &mut houses));

    houses.len()
}

fn follow_dir(dir: &str, pos: &mut (isize, isize), hist: &mut HashSet<(isize, isize)>) {
    let (x, y) = *pos;

    match dir {
        "<" => *pos = (x - 1, y),
        ">" => *pos = (x + 1, y),
        "^" => *pos = (x, y + 1),
        "v" => *pos = (x, y - 1),
        _ => (),
    }

    hist.insert(*pos);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input"), 2572);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input"), 2631);
    }
}
