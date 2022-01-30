use regex::Regex;
use std::collections::HashMap;
//use std::fs;

pub type Menu = HashMap<String, (usize, Vec<Chemical>)>;

pub struct Chemical {
    name: String,
    qty: usize
}

impl Chemical {
    fn new(name: &str, qty: usize) -> Self {
        Chemical {
            name: name.to_string(),
            qty
        }
    }

    fn from_str(s: &str) -> Self {
        let mut s = s.trim().split(' ');
        let qty = s.next().unwrap().parse::<usize>().unwrap();
        let name = s.next().unwrap().to_string();
        Chemical {
            name,
            qty
        }
   }
}

fn ceiling_div(a: usize, b: usize) -> usize {
    a / b + (if a % b != 0 {1} else {0})
}

//31 for x small, 165 for small
//42108 < and < 843541 for full
fn hunt(target: Chemical, recipes: &Menu) -> usize {
    /*
     * Unfortunately this recursive approach fails to identify the minimum requirement,
     * since, although the data structure is not cyclic, it is possible for multiple parents
     * to share a child. e.g:

     "3 ORE => 2 A", "1 A => 1 B", "1 A => 1 C", "1 B, 1 C => 1 FUEL" => 3 ORE

     * approach: use two stacks, plus an ore counter.
     * pop stack and check if spares can (even partially!) fulfil the requirement;
     * if spares exceed demand, substract requirement from spares.
     * if spares equal demand, subtract and pop from spares.
     * if demand exceeds spares or no spares, subtract and continue.
     *     calculate minimum number of orders (i.e. ceiling division)
     *     look up recipe, mult by number of orders,
     *     push onto stack, collapsing as you go.
     */
    let mut requirements = vec![target];
    let mut spares: Vec<Chemical> = vec![];
    let mut ore = 0;
    while !requirements.is_empty() {
        let order = requirements.pop().unwrap();
        if spares.find(&order) {
        } else {
        }
    }

    ore as usize
}

pub fn get_input() -> Menu {
    let mut recipes = HashMap::new();

    for l in include_str!("../../data/14.small.txt").lines() {
        let mut s = l.split(" => ");
        let inputs = s.next().unwrap().split(',').map(|i| Chemical::from_str(i)).collect();
        let output = Chemical::from_str(s.next().unwrap());
        recipes.insert(output.name, (output.qty, inputs));
    }
    recipes
}

pub fn part_1(input: &Menu) -> usize {
    let target = Chemical::new("FUEL", 1);
    hunt(target, input)
}
