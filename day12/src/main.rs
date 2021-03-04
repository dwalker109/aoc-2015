use std::{fs, io};

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> i64 {
    let file = fs::File::open(path).unwrap();
    let rdr = io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr).unwrap();

    descend(&json, 0, false)
}

fn part2(path: &str) -> i64 {
    let file = fs::File::open(path).unwrap();
    let rdr = io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(rdr).unwrap();

    descend(&json, 0, true)
}

fn descend(field: &serde_json::Value, acc: i64, skip_red: bool) -> i64 {
    if field.is_number() {
        field.as_i64().unwrap()
    } else if field.is_array() {
        field
            .as_array()
            .unwrap()
            .iter()
            .map(|x| descend(x, acc, skip_red))
            .sum()
    } else if field.is_object() {
        let obj = field.as_object().unwrap();
        if skip_red
            && obj
                .values()
                .filter(|&x| x.is_string())
                .any(|x| x.as_str().unwrap() == "red")
        {
            return acc;
        }
        obj.iter().map(|(_, x)| descend(x, acc, skip_red)).sum()
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input_test"), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input_test"), 9);
    }
}
