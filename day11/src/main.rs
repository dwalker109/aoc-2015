fn main() {
    let p1 = next_password("cqjxjnds");
    let p2 = next_password(&format!("{}", p1));

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

mod password;

fn next_password(input: &str) -> password::Password {
    let mut password = password::Password::from(input);

    loop {
        password.increment();

        if password.has_increasing_straight()
            && password.contains_no_mistakeables()
            && password.has_two_pairs()
        {
            return password;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(next_password("abcdefgh"), "abcdffaa");
    }

    #[test]
    fn test_part2() {
        assert_eq!(next_password("cqjxxyzz"), "cqkaabcc");
    }
}
