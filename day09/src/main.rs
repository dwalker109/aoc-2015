use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let p1 = part1("./input");
    let p2 = part2("./input");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str) -> u32 {
    *get_routes(path).first().unwrap()
}

fn part2(path: &str) -> u32 {
    *get_routes(path).last().unwrap()
}

fn get_routes(path: &str) -> Vec<u32> {
    let data = read_to_string(path).unwrap();

    let (locations, distances) = parse_input(&data);
    let permuations = get_permutations(locations.into_iter().collect());

    let mut routes = permuations
        .iter()
        .map(|route| {
            route.windows(2).fold(0, |acc, cities| {
                let city_a = cities[0];
                let city_b = cities[1];

                acc + *distances.get(&(city_a, city_b)).unwrap()
            })
        })
        .collect::<Vec<_>>();

    routes.sort_unstable();

    routes
}

fn parse_input(raw: &str) -> (HashSet<&str>, HashMap<(&str, &str), u32>) {
    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    for line in raw.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let city_a = parts[0];
        let city_b = parts[2];
        let distance = parts[4].parse().unwrap();

        locations.insert(city_a);
        locations.insert(city_b);
        distances.insert((city_a, city_b), distance);
        distances.insert((city_b, city_a), distance);
    }

    (locations, distances)
}

/// This function is completely pointless (I could have used itertools::permutations)
/// but since this is AoC and I spent ages learning how to do this, I'll use it
fn get_permutations(all_items: Vec<&str>) -> Vec<Vec<&str>> {
    let mut collected: Vec<Vec<&str>> = vec![];

    fn recurse<'a, 'b>(
        collected: &'b mut Vec<Vec<&'a str>>,
        current: Vec<&'a str>,
        remaining: Vec<&'a str>,
    ) {
        if remaining.len() != 0 {
            for (i, &next) in remaining.iter().enumerate() {
                let mut it_remaining = remaining.clone();
                let mut it_current = current.clone();

                it_remaining.remove(i);
                it_current.push(next);

                recurse(collected, it_current, it_remaining);
            }
        } else {
            collected.push(current.clone());
        }
    }

    recurse(&mut collected, vec![], all_items);

    collected
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input_test"), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input_test"), 982);
    }
}
