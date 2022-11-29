use std::cmp::max;

pub struct Ingredient<'a> {
    name: &'a str,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize
}

impl<'a> Ingredient<'a> {
    fn new(input: &'a str) -> Self {
        let input = input.split(':').collect::<Vec<&'a str>>();
        let properties = input[1].split(',').collect::<Vec<&'a str>>();
        Self {
            name: input[0],
            capacity : properties[0].split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
            durability : properties[1].split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
            flavor : properties[2].split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
            texture : properties[3].split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
            calories : properties[4].split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
        }
    }
}

struct Cookie<'a> {
    ingredients: Vec<Ingredient<'a>>
}

impl Cookie<'_> {
    fn score(&self) -> usize {
        let capacity = max(self.ingredients.iter().map(|i| i.capacity).sum::<isize>(), 0) as usize;
        let durability = max(self.ingredients.iter().map(|i| i.durability).sum::<isize>(), 0) as usize;
        let flavor = max(self.ingredients.iter().map(|i| i.texture).sum::<isize>(), 0) as usize;
        let texture = max(self.ingredients.iter().map(|i| i.flavor).sum::<isize>(), 0) as usize;
        capacity * durability * flavor * texture
    }
}

pub fn get_input() -> Vec<Ingredient<'static>> {
    include_str!("../../data/15.txt")
        .lines()
        .map(Ingredient::new)
        .collect()
}

pub fn part_1(_input: &[Ingredient]) -> usize {
    todo!()
}

pub fn part_2(_input: &[Ingredient]) -> usize {
    todo!()
}
