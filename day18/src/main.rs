use std::{collections::HashMap, fs::read_to_string};

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
    state: HashMap<(isize, isize), bool>,
    stuck: Vec<(isize, isize)>,
}

impl<'a> Grid<'a> {
    fn from(raw: &str, with_stuck: bool) -> Grid {
        let mut initial_state = HashMap::new();

        for (y, line) in raw.lines().enumerate() {
            for (x, light) in line.chars().enumerate() {
                initial_state.insert((y as isize, x as isize), light == '#');
            }
        }

        let stuck = match with_stuck {
            true => {
                let y_max = (raw.lines().count() - 1) as isize;
                let x_max = (raw.lines().next().unwrap().len() - 1) as isize;
                let stuck = vec![(0, 0), (0, x_max), (y_max, 0), (y_max, x_max)];
                for s in stuck.iter() {
                    initial_state.insert(*s, true);
                }
                stuck
            }
            false => {
                vec![]
            }
        };

        Grid {
            _raw: raw,
            state: initial_state,
            stuck,
        }
    }

    fn gen_next_state(&mut self) {
        let mut next_state = self.state.clone();

        for ((y, x), lit) in self.state.iter() {
            let lit_neighbours_qty = (y - 1..=y + 1)
                .flat_map(|ny| (x - 1..=x + 1).map(move |nx| (ny, nx)))
                .filter(|nyx| nyx != &(*y, *x))
                .fold(0, |acc, n| {
                    if let Some(true) = self.state.get(&n) {
                        acc + 1
                    } else {
                        acc
                    }
                });

            let light_next_state = match lit_neighbours_qty {
                2 | 3 if *lit => true,
                3 if !*lit => true,
                _ => false,
            };

            next_state.insert((*y, *x), light_next_state);
        }

        for s in self.stuck.iter() {
            next_state.insert(*s, true);
        }

        self.state = next_state;
    }

    fn count_lit_lights(&self) -> usize {
        self.state.iter().filter(|((_, _), &lit)| lit).count()
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
