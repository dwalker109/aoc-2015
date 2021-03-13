use std::{cmp::min, collections::HashSet, fs::read_to_string};

fn main() {
    let p1 = game_of_life("./input", 100, false);
    let p2 = game_of_life("./input", 100, true);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn game_of_life(path: &str, steps: u32, with_stuck: bool) -> usize {
    let data = read_to_string(path).unwrap();
    let mut grid = Grid::from(&data, with_stuck);

    let mut step = 0;
    while step < steps {
        grid.gen_next_state();
        step += 1;
    }

    grid.count_lit_lights()
}

struct Grid<'a> {
    _raw: &'a str,
    y_max: usize,
    x_max: usize,
    state: HashSet<(usize, usize)>,
    stuck: Vec<(usize, usize)>,
}

impl<'a> Grid<'a> {
    fn from(raw: &str, with_stuck: bool) -> Grid {
        let y_max = raw.lines().count() - 1;
        let x_max = raw.lines().next().unwrap().len() - 1;

        let mut initial_state = HashSet::with_capacity((y_max * x_max) as usize);

        for (y, line) in raw.lines().enumerate() {
            for (x, light) in line.chars().enumerate() {
                if light == '#' {
                    initial_state.insert((y, x));
                }
            }
        }

        let stuck = match with_stuck {
            true => {
                let stuck = vec![(0, 0), (0, x_max), (y_max, 0), (y_max, x_max)];
                for s in stuck.iter() {
                    initial_state.insert(*s);
                }
                stuck
            }
            false => {
                vec![]
            }
        };

        Grid {
            _raw: raw,
            y_max,
            x_max,
            state: initial_state,
            stuck,
        }
    }

    fn gen_next_state(&mut self) {
        let mut next_state = HashSet::with_capacity(self.state.capacity());

        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                let lit = self.state.contains(&(y, x));

                let lit_neighbours_qty = (y.saturating_sub(1)..=min(self.y_max, y + 1))
                    .flat_map(|ny| {
                        (x.saturating_sub(1)..=min(self.x_max, x + 1)).map(move |nx| (ny, nx))
                    })
                    .filter(|nyx| nyx != &(y, x))
                    .fold(0, |acc, n| {
                        if self.state.contains(&n) {
                            acc + 1
                        } else {
                            acc
                        }
                    });

                let should_light = match lit_neighbours_qty {
                    2 | 3 if lit => true,
                    3 if !lit => true,
                    _ => false,
                };

                if should_light {
                    next_state.insert((y, x));
                }
            }
        }

        next_state.extend(self.stuck.iter());

        self.state = next_state;
    }

    fn count_lit_lights(&self) -> usize {
        self.state.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(game_of_life("./input_test", 4, false), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(game_of_life("./input_test", 5, true), 17);
    }
}
