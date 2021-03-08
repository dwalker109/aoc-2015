use std::{cmp, fs};

fn main() {
    let p1 = part1("./input", 2503);
    let p2 = part2("./input", 2503);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(path: &str, duration: u32) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let mut reindeers = parse_input(&input);

    for _ in 0..duration {
        reindeers.iter_mut().for_each(|r| r.tick())
    }

    reindeers
        .iter()
        .fold(0, |acc, r| cmp::max(acc, r.travelled))
}

fn part2(path: &str, duration: u32) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let mut reindeers = parse_input(&input);
    let mut max_travelled = 0;

    for _ in 0..duration {
        reindeers.iter_mut().for_each(|r| {
            r.tick();
            max_travelled = cmp::max(max_travelled, r.travelled);
        });
        reindeers
            .iter_mut()
            .filter(|r| r.travelled == max_travelled)
            .for_each(|r| r.points += 1);
    }

    reindeers.iter().fold(0, |acc, r| cmp::max(acc, r.points))
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
    travelled: u32,
    is_resting: bool,
    countdown: u32,
    points: u32,
}

impl Reindeer {
    fn tick(&mut self) {
        self.countdown -= 1;

        if !self.is_resting {
            self.travelled += self.speed;
        }

        if self.countdown == 0 {
            self.is_resting = !self.is_resting;
            self.countdown = match self.is_resting {
                true => self.rest,
                false => self.duration,
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let regex = regex::Regex::new(
        r"\s*(?P<name>\w*)\D*(?P<speed>\d*)\D*(?P<duration>\d*)\D*(?P<rest>\d*).*\s*",
    )
    .unwrap();

    let reindeers = regex
        .captures_iter(input)
        .map(|cap| Reindeer {
            name: cap["name"].to_string(),
            speed: cap["speed"].parse().unwrap(),
            duration: cap["duration"].parse().unwrap(),
            rest: cap["rest"].parse().unwrap(),
            travelled: 0,
            is_resting: false,
            countdown: cap["duration"].parse().unwrap(),
            points: 0,
        })
        .collect();

    reindeers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./input_test", 1000), 1120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./input_test", 1000), 689);
    }
}
