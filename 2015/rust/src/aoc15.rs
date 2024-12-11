use std::cmp::max;

const TOTAL: usize = 100;

pub struct Ingredient<'a> {
    name: &'a str,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl<'a> Ingredient<'a> {
    fn new(input: &'a str) -> Self {
        let input = input.split(':').collect::<Vec<&'a str>>();
        let properties = input[1].split(',').collect::<Vec<&'a str>>();
        Self {
            name: input[0],
            capacity: properties[0]
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
            durability: properties[1]
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
            flavor: properties[2]
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
            texture: properties[3]
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
            calories: properties[4]
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<isize>()
                .unwrap(),
        }
    }
}

struct Cookie<'a> {
    ingredients: Vec<(&'a Ingredient<'a>, usize)>,
}

impl<'a> Cookie<'a> {
    fn new(amounts: Vec<usize>, input: &'a [Ingredient]) -> Self {
        let ingredients = input.iter().zip(amounts).collect::<Vec<_>>();
        Self { ingredients }
    }

    fn score(&self) -> isize {
        let capacity = max(
            self.ingredients
                .iter()
                .map(|i| i.0.capacity * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let durability = max(
            self.ingredients
                .iter()
                .map(|i| i.0.durability * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let flavor = max(
            self.ingredients
                .iter()
                .map(|i| i.0.flavor * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let texture = max(
            self.ingredients
                .iter()
                .map(|i| i.0.texture * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        capacity * durability * flavor * texture
    }

    fn score_with_calories(&self) -> Option<isize> {
        let capacity = max(
            self.ingredients
                .iter()
                .map(|i| i.0.capacity * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let durability = max(
            self.ingredients
                .iter()
                .map(|i| i.0.durability * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let flavor = max(
            self.ingredients
                .iter()
                .map(|i| i.0.flavor * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let texture = max(
            self.ingredients
                .iter()
                .map(|i| i.0.texture * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        let calories = max(
            self.ingredients
                .iter()
                .map(|i| i.0.calories * i.1 as isize)
                .sum::<isize>(),
            0,
        );
        if calories == 500 {
            Some(capacity * durability * flavor * texture)
        } else {
            None
        }
    }
}

fn get_combinations(n: usize, total: usize) -> Vec<Vec<usize>> {
    let mut out = vec![];
    if n == 1 {
        out.push(vec![total]);
    } else {
        for i in 0..=total {
            for c in get_combinations(n - 1, total - i).iter_mut() {
                let mut v = vec![i];
                v.append(c);
                out.push(v);
            }
        }
    }
    out
    /*
    (0..=total)
        .map(|i| {
            if n == 1 {
                vec![vec![0usize]].into_iter().map(|_| vec![i])
            } else {
                get_combinations(n - 1, total - i).into_iter()
                    .map(|c| {let mut v = vec![i]; v.append(&mut c); v})
            }
        })
        .collect::<Vec<_>>()
    */
}

pub fn get_input() -> Vec<Ingredient<'static>> {
    include_str!("../../data/15.txt")
        .lines()
        .map(Ingredient::new)
        .collect()
}

pub fn part_1(input: &[Ingredient]) -> isize {
    let n = input.len();
    let options = get_combinations(n, TOTAL);
    options
        .into_iter()
        .map(|o| {
            let c = Cookie::new(o, input);
            c.score()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &[Ingredient]) -> isize {
    let n = input.len();
    let options = get_combinations(n, TOTAL);
    options
        .into_iter()
        .filter_map(|o| {
            let c = Cookie::new(o, input);
            c.score_with_calories()
        })
        .max()
        .unwrap()
}
