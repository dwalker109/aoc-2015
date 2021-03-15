use std::{cmp::max, fs};

fn main() {
    let p1 = bake("./input", false);
    let p2 = bake("./input", true);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn bake(path: &str, weight_watching: bool) -> isize {
    let data = fs::read_to_string(path).unwrap();
    let ingredients = parse_input(&data);
    let sum_permutations = gen_permutations(ingredients.len(), 100);
    let mut scores = most_tastiest(&sum_permutations, &ingredients, weight_watching);

    scores.sort_unstable();
    *scores.last().unwrap()
}

fn most_tastiest(
    sum_permutations: &[Vec<usize>],
    ingredients: &[Ingredient],
    five_hundred_cals_target: bool,
) -> Vec<isize> {
    sum_permutations
        .iter()
        .map(|sp| {
            if five_hundred_cals_target {
                let calories = sp.iter().enumerate().fold(0, |acc, (n, qty)| {
                    acc + ingredients[n].calories * (*qty as isize)
                });
                if calories != 500 {
                    return 0;
                }
            }

            let capacity = sp.iter().enumerate().fold(0, |acc, (n, qty)| {
                acc + ingredients[n].capacity * (*qty as isize)
            });
            let durability = sp.iter().enumerate().fold(0, |acc, (n, qty)| {
                acc + ingredients[n].durability * (*qty as isize)
            });
            let flavor = sp.iter().enumerate().fold(0, |acc, (n, qty)| {
                acc + ingredients[n].flavor * (*qty as isize)
            });
            let texture = sp.iter().enumerate().fold(0, |acc, (n, qty)| {
                acc + ingredients[n].texture * (*qty as isize)
            });

            max(capacity, 0) * max(durability, 0) * max(flavor, 0) * max(texture, 0)
        })
        .collect()
}

struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_input(data: &str) -> Vec<Ingredient> {
    data.lines()
        .map(|l| {
            let w = l.split_ascii_whitespace().collect::<Vec<_>>();

            Ingredient {
                capacity: w[2].strip_suffix(",").unwrap().parse().unwrap(),
                durability: w[4].strip_suffix(",").unwrap().parse().unwrap(),
                flavor: w[6].strip_suffix(",").unwrap().parse().unwrap(),
                texture: w[8].strip_suffix(",").unwrap().parse().unwrap(),
                calories: w[10].parse().unwrap(),
            }
        })
        .collect()
}

fn gen_permutations(qty: usize, sum: usize) -> Vec<Vec<usize>> {
    fn recurse(mut acc: Vec<usize>, level: usize, remainder: usize) -> Vec<Vec<usize>> {
        match level {
            1 => {
                if remainder == 0 {
                    vec![] // Gone too far, empty vec will be flattened out
                } else {
                    acc.push(remainder);
                    vec![acc]
                }
            }
            _ => (1..=remainder)
                .flat_map(|n| {
                    let mut lcl_acc = acc.clone();
                    lcl_acc.push(n);
                    recurse(lcl_acc, level - 1, remainder - n)
                })
                .collect(),
        }
    }

    recurse(Vec::with_capacity(qty), qty, sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(62842880, bake("./input_test", false));
    }
    #[test]
    fn test_part2() {
        assert_eq!(57600000, bake("./input_test", true));
    }
}
