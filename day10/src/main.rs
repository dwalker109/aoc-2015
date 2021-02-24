fn main() {
    let p1 = part1(&mut "1113122113");
    let p2 = part2(&mut "1113122113");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(input: &str) -> usize {
    run_x_times(input, 40)
}

fn part2(input: &str) -> usize {
    run_x_times(input, 50)
}

fn run_x_times(input: &str, qty: u32) -> usize {
    let mut progress = input.to_string();

    for _ in 0..qty {
        progress = look_and_say(&progress);
    }

    progress.len()
}

fn look_and_say(source: &str) -> String {
    let source_bytes = source.as_bytes();

    let mut tally: Vec<(u32, u8)> = vec![];
    let mut count = 0;

    for n in 0..source.len() {
        let curr = source_bytes[n];

        count += 1;

        if n + 1 == source.len() {
            tally.push((count, curr));
            break;
        }

        let next = source_bytes[n + 1];

        if curr != next {
            tally.push((count, curr));
            count = 0;
        }
    }

    tally.iter().fold(String::new(), |acc, &(len, curr)| {
        [acc, len.to_string(), curr.to_string()].concat()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(look_and_say("111221").len(), 6);
    }
}
