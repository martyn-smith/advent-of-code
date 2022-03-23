use regex::Regex;
use std::collections::HashMap;
//use std::fs;

//type Order = (String, usize);
pub type Menu = HashMap<String, (usize, Vec<Order>)>;

#[derive(Debug)]
pub struct Order {
    name: String,
    qty: usize
}

impl Order {
    fn new(name: &str, qty: usize) -> Self {
        Order {
            name: name.to_string(),
            qty
        }
    }

    fn from_str(s: &str) -> Self {
        let mut s = s.trim().split(' ');
        let qty = s.next().unwrap().parse::<usize>().unwrap();
        let name = s.next().unwrap().to_string();
        Order {
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
fn hunt(target: Order, recipes: &Menu) -> usize {
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
    let mut spares: HashMap<String, usize> = HashMap::new();
    let mut ore = 0;
    while !requirements.is_empty() {
        dbg!(&requirements);
        let mut r = requirements.pop().unwrap();
        dbg!(&r);
        dbg!(&spares);
        if let Some(s) = spares.get_mut(&r.name) {
            if r.qty >= *s {
                r.qty -= *s;
                *s = 0;
            } else {
                *s -= r.qty;
                r.qty = 0;
            }
        }
        //cleanup empty spares
        if spares.contains_key(&r.name) && *spares.get(&r.name).unwrap() == 0 {
            spares.remove(&r.name);
        }
        if r.qty == 0 {
            continue;
        }
        let order = recipes.get(&r.name).expect(&format!("{} not found", r.name));
        let n = ceiling_div(r.qty, order.0);
        for c in order.1.iter() {
            let spare = (order.0 * n) - r.qty;
            //add to spares
            if let Some(s) = spares.get_mut(&c.name) {
                *s += spare;
            } else {
                spares.insert(c.name.to_string(), spare);
                if c.name == "ORE" {
                    ore += c.qty;
                } else {
                    requirements.push(Order {name: c.name.to_string(), qty: c.qty});
                }
            }
        }
    }
    ore as usize
}

pub fn get_input() -> Menu {
    let mut recipes = HashMap::new();

    for l in include_str!("../../data/14.vsmall.txt").lines() {
        let mut s = l.split(" => ");
        let inputs = s.next().unwrap().split(',').map(|i| Order::from_str(i)).collect();
        let output = Order::from_str(s.next().unwrap());
        recipes.insert(output.name, (output.qty, inputs));
    }
    recipes
}

pub fn part_1(input: &Menu) -> usize {
    let target = Order::new("FUEL", 1);
    hunt(target, input)
}
