const INPUT: &str = "1113122113";

fn main() {
    let p1 = part1(INPUT);
    println!("Part 1: {}", p1);

    let p2 = part2(INPUT);
    println!("Part 2: {}", p2);
}

fn part1(input: &str) -> usize {
    run_x_times(input, 40)
}

fn part2(input: &str) -> usize {
    run_x_times(input, 50)
}

fn run_x_times(input: &str, qty: u32) -> usize {
    let mut progress = Vec::from(input.as_bytes());

    for _ in 0..qty {
        progress = look_and_say(&progress);
    }

    progress.len()
}

fn look_and_say(source_bytes: &[u8]) -> Vec<u8> {
    let mut tally: Vec<u8> = Vec::new();
    let mut count: u32 = 0;

    fn push_to_tally(tally: &mut Vec<u8>, count: &u32, curr: &u8) {
        for b in count.to_string().as_bytes().iter() {
            tally.push(*b);
        }
        tally.push(*curr);
    };

    for n in 0..source_bytes.len() {
        let curr = source_bytes.get(n).unwrap();

        count += 1;

        if n + 1 == source_bytes.len() {
            push_to_tally(&mut tally, &count, &curr);
            break;
        }

        let next = source_bytes.get(n + 1).unwrap();

        if curr != next {
            push_to_tally(&mut tally, &count, &curr);
            count = 0;
        }
    }

    tally
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 360154);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5103798);
    }
}
