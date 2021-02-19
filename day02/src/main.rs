fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> usize {
    let dat = std::fs::read_to_string(path).unwrap();
    let area = dat.split_terminator("\n").map(calc_wrapping).sum();
    area
}

fn part2(path: &str) -> usize {
    let dat = std::fs::read_to_string(path).unwrap();
    let distance = dat.split_terminator("\n").map(calc_ribbon).sum();
    distance
}

fn calc_wrapping(dimensions: &str) -> usize {
    let (l, w, h) = parse_dims(dimensions);
    let mut sides = vec![l * w, w * h, h * l];
    sides.sort_unstable();

    2 * sides[0] + 2 * sides[1] + 2 * sides[2] + sides[0]
}

fn calc_ribbon(dimensions: &str) -> usize {
    let (l, w, h) = parse_dims(dimensions);
    let mut sides = vec![l, w, h];
    sides.sort_unstable();

    sides[0] + sides[0] + sides[1] + sides[1] + (l * w * h)
}

fn parse_dims(dimensions: &str) -> (usize, usize, usize) {
    let dims: Vec<_> = dimensions
        .splitn(3, "x")
        .map(|dim| dim.parse::<usize>().unwrap())
        .collect();

    (dims[0], dims[1], dims[2])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input"), 1588178);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input"), 3783758);
    }
}
