fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> isize {
    let mut floor = 0;
    let dat = std::fs::read_to_string(path).unwrap();

    dat.split("").for_each(|elem| match elem {
        "(" => floor += 1,
        ")" => floor -= 1,
        _ => (),
    });

    floor
}

fn part2(path: &str) -> usize {
    let mut floor = 0;
    let dat = std::fs::read_to_string(path).unwrap();
    let items: Vec<(usize, &str)> = dat.split("").enumerate().collect();

    for (n, elem) in items {
        match elem {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return n;
        }
    }

    0
}
