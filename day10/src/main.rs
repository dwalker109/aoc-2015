use std::cell::RefCell;

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
    let tally = RefCell::new(Vec::<Vec<u8>>::with_capacity(10_000_000));
    let count = RefCell::new(0 as u32);

    let push_to_tally = |curr: &u8| {
        tally
            .borrow_mut()
            .push(count.borrow().to_string().as_bytes().to_vec());
        tally.borrow_mut().push(vec![*curr]);
    };

    for n in 0..source_bytes.len() {
        let curr = source_bytes[n];

        *count.borrow_mut() += 1;

        if n + 1 == source_bytes.len() {
            push_to_tally(&curr);
            break;
        }

        let next = source_bytes[n + 1];

        if curr != next {
            push_to_tally(&curr);
            *count.borrow_mut() = 0;
        }
    }

    let joined: Vec<u8> = tally.borrow().to_owned().into_iter().flatten().collect();

    joined
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
