use std::fs;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> usize {
    let input = get_input(path);

    let literal_length: usize = input.iter().map(|line| line.len()).sum();

    let esc_backslash = regex::Regex::new(r"\\\\").unwrap();
    let esc_doublequote = regex::Regex::new(r#"\\""#).unwrap();
    let esc_hexchar = regex::Regex::new(r"\\x.{2}").unwrap();

    let inmem_length: usize = input
        .iter()
        .map(|line| {
            let mut inmem = line.clone();
            inmem = esc_backslash.replace_all(&inmem, "$").to_string();
            inmem = esc_doublequote.replace_all(&inmem, "$").to_string();
            inmem = esc_hexchar.replace_all(&inmem, "$").to_string();

            inmem[1..inmem.len() - 1].len()
        })
        .sum();

    literal_length - inmem_length
}

fn part2(path: &str) -> usize {
    let input = get_input(path);

    let literal_length: usize = input.iter().map(|line| line.len()).sum();

    let inmem_length: usize = input
        .iter()
        .map(|line| {
            let inmem = line.replace(r"\", r"$$").replace(r#"""#, r"$$");
            
            format!("${}$", inmem).len()
        })
        .sum();

    inmem_length - literal_length
}

fn get_input(path: &str) -> Vec<String> {
    let data = fs::read_to_string(path).expect("Could not get input");
    let input: Vec<String> = data.lines().map(|line| line.to_owned()).collect();

    input
}

mod test {
    use super::*;    

    #[test]
    fn test_part1() {
        let result = part1("./input");
        assert_eq!(result, 1333);
    }
    
    #[test]
    fn test_part2() {
        let result = part2("./input");
        assert_eq!(result, 2046);
    }
}
