const FIRST_CODE: usize = 20151125;
const CODE_MULT: usize = 252533;
const CODE_REM: usize = 33554393;

fn main() {
    let p1 = find_code((2981, 3075));

    println!("Part 1: {}", p1);
}

fn find_code(target: (usize, usize)) -> usize {
    let mut curr_rc = (1usize, 1usize);
    let mut max_row = 1usize;
    let mut last_code = FIRST_CODE;

    loop {
        let (next_curr_rc, next_max_row) = next_rc(curr_rc, max_row);
        curr_rc = next_curr_rc;
        max_row = next_max_row;

        let code = next_code(last_code);
        last_code = code;

        if curr_rc == target {
            break code;
        }
    }
}

fn next_code(n: usize) -> usize {
    (n * CODE_MULT) % CODE_REM
}

fn next_rc(curr: (usize, usize), max_row: usize) -> ((usize, usize), usize) {
    let next = (curr.0 - 1, curr.1 + 1);

    if next.0 == 0 {
        ((max_row + 1, 1), max_row + 1)
    } else {
        (next, max_row)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_calc_next_code() {
        assert_eq!(next_code(FIRST_CODE), 31916031);
    }

    #[test]
    fn can_find_a_code() {
        assert_eq!(find_code((4, 6)), 31527494);
    }
}
