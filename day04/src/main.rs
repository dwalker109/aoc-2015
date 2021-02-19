extern crate crypto;
use crypto::{digest::Digest, md5::Md5};

const SEC_KEY: &str = "yzbqklnj";

fn main() {
    let p1 = part1(SEC_KEY);
    let p2 = part2(SEC_KEY);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(sk: &str) -> isize {
    mine(sk, "00000")
}

fn part2(sk: &str) -> isize {
    mine(sk, "000000")
}

fn mine(sk: &str, lead_in: &str) -> isize {
    let mut md5 = Md5::new();
    md5.input_str(sk);

    for n in 0.. {
        let mut lmd5 = md5.clone();
        lmd5.input_str(&n.to_string());
        if lmd5.result_str().starts_with(lead_in) {
            return n;
        }
    }

    panic!("gahhhh");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("pqrstuv"), 5714438);
    }
}
