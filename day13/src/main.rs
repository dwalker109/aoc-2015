use std::{
    collections::{hash_set::Iter, HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> i32 {
    let input = read_to_string(path).unwrap();

    let (lookup, people) = parse_input(&input);
    let permutations = people.iter().permutations(people.len());
    let scores = calc_scores(permutations, &lookup);

    *scores.last().unwrap()
}

fn part2(path: &str) -> i32 {
    let input = read_to_string(path).unwrap();

    let (mut lookup, mut people) = parse_input(&input);
    let (lookup_with_me, people_with_me) = invite_me(&mut lookup, &mut people, "dan");
    let permutations = people_with_me.iter().permutations(people_with_me.len());
    let scores = calc_scores(permutations, lookup_with_me);

    *scores.last().unwrap()
}

fn parse_input(input: &str) -> (HashMap<(&str, &str), i32>, HashSet<&str>) {
    let mut lookup = HashMap::new();
    let mut people = HashSet::new();

    for line in input.lines() {
        let words = line
            .strip_suffix('.')
            .unwrap()
            .split_ascii_whitespace()
            .collect_vec();

        let name_a = words[0];
        let name_b = words[10];

        let sign = words[2];
        let qty = words[3].parse::<i32>().unwrap();
        let hmod = match sign {
            "gain" => qty,
            "lose" => qty.checked_neg().unwrap(),
            _ => 0,
        };

        lookup.insert((name_a, name_b), hmod);
        people.insert(name_a);
        people.insert(name_b);
    }

    (lookup, people)
}

fn invite_me<'a>(
    lookup: &'a mut HashMap<(&'a str, &'a str), i32>,
    people: &'a mut HashSet<&'a str>,
    me: &'a str,
) -> (&'a HashMap<(&'a str, &'a str), i32>, &'a HashSet<&'a str>) {
    for person in people.iter() {
        lookup.insert((person, me), 0);
        lookup.insert((me, person), 0);
    }
    people.insert(me);

    (lookup, people)
}

fn calc_scores(
    permutations: itertools::Permutations<Iter<&str>>,
    lookup: &HashMap<(&str, &str), i32>,
) -> Vec<i32> {
    let scores = permutations
        .map(|p| {
            let mut x = p.clone();
            x.push(&x[0]);
            x.windows(2).fold(0, |acc, y| {
                let name_a = y[0];
                let name_b = y[1];

                acc + lookup.get(&(name_a, name_b)).unwrap()
                    + lookup.get(&(name_b, name_a)).unwrap()
            })
        })
        .sorted()
        .collect_vec();

    scores
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input_test"), 330);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input_test"), 286);
    }
}
