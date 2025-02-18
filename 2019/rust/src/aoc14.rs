use std::cmp::min;
use std::collections::HashMap;

pub type Reactions = Vec<(Component, Vec<Component>)>;

const ORE: usize = 1_000_000_000_000;

#[derive(Clone, Debug)]
pub struct Component {
    name: String,
    qty: usize,
}

impl Component {
    fn new(name: &str, qty: usize) -> Self {
        Component {
            name: name.to_string(),
            qty,
        }
    }

    fn from_str(s: &str) -> Self {
        let mut s = s.trim().split(' ');
        let qty = s.next().unwrap().parse::<usize>().unwrap();
        let name = s.next().unwrap().to_string();
        Component { name, qty }
    }
}

fn take_spares(mut r: Component, spares: &mut [Component]) -> Option<Component> {
    if let Some(c) = spares.iter_mut().find(|c| c.name == r.name) {
        let qty = min(r.qty, c.qty);
        r.qty -= qty;
        c.qty -= qty;
    }
    if r.qty > 0 {
        Some(r)
    } else {
        None
    }
}

fn get_order(r: Component, reactions: &Reactions, spares: &mut Vec<Component>) -> Vec<Component> {
    let order = reactions
        .iter()
        .find(|c| c.0.name == r.name)
        .expect("no reaction found");
    let mut qty = order.0.qty;
    let mut order = order.1.to_owned();
    if r.qty % qty > 0 {
        let o = qty;
        qty = r.qty / qty + 1;
        let spare = Component::new(&r.name[..], (qty * o) - r.qty);
        spares.push(spare);
    } else {
        qty = r.qty / qty;
    }
    for o in order.iter_mut() {
        o.qty *= qty;
    }
    order
}

fn hunt(amount: usize, spares: &mut Vec<Component>, reactions: &Reactions) -> usize {
    /*
    * Initially, I wanted to try a recursive approach.
    * However this failed to identify the minimum requirement since,
    * although the data structure is not cyclic, it is possible for multiple parents
    * to share a child. e.g:

    "3 ORE => 2 A", "1 A => 1 B", "1 A => 1 C", "1 B, 1 C => 1 FUEL" => 3 ORE

    * New (and community-accepted) approach: use two stacks, plus an ore counter.
    * Pop stack and check if spares can (even partially!) fulfil the requirement;
    * if spares exceed demand, substract requirement from spares.
    * if spares equal demand, subtract and pop from spares.
    * if demand exceeds spares or no spares, subtract and continue.
    *     calculate minimum number of orders (i.e. ceiling division)
    *     look up recipe, mult by number of orders,
    *     push onto stack, collapsing as you go.
    * TODO: I implemented Reactions and Spares as Vectors to improve readbility,
    * reasoning the performance penalty is low (possibly even negative?),
    * check this and reimplement HashMaps
    */
    let mut requirements = vec![Component::new("FUEL", amount)];
    let mut ore = 0;
    while let Some(r) = requirements.pop() {
        //println!(
        //    "ORE: {}\nrequired: {}\nspare: {}\n",
        //    ore,
        //    requirements.iter().fold(String::new(), |mut out, e| {
        //        let _ = write!(&mut out, "{0} {1}, ", e.qty, e.name);
        //        out
        //    }),
        //    spares.iter().fold(String::new(), |mut out, e| {
        //        let _ = write!(&mut out, "{0} {1}, ", e.qty, e.name);
        //        out
        //    })
        //);
        //let r = requirements.pop().unwrap();
        if let Some(r) = take_spares(r, spares) {
            let order = get_order(r, reactions, spares);
            for o in order.into_iter() {
                if let Some(r) = requirements.iter_mut().find(|r| r.name == o.name) {
                    r.qty += o.qty;
                } else {
                    requirements.push(o);
                }
            }
            // other than the HashMap, one of the few differences from
            // https://github.com/prscoelho 's approach (indepent work, honest!)
            // is I put the ore collation here instead of popping.
            for r in requirements.iter_mut() {
                if r.name == "ORE" {
                    ore += r.qty;
                    r.qty = 0;
                }
            }
        }
        requirements.retain(|r| r.qty > 0);
        spares.retain(|s| s.qty > 0);
    }
    ore
}

//fn produce(ore: &mut usize, spares: &mut Vec<Component>, reactions: &Reactions) -> usize {
//
//}

pub fn get_input() -> Reactions {
    let mut recipes = Vec::new();

    for l in include_str!("../../data/14.txt").lines() {
        let mut s = l.split(" => ");
        let inputs = s
            .next()
            .unwrap()
            .split(',')
            .map(Component::from_str)
            .collect();
        let output = Component::from_str(s.next().unwrap());
        recipes.push((output, inputs));
    }
    recipes
}

pub fn part_1(input: &Reactions) -> usize {
    let mut spares = Vec::<Component>::new();
    hunt(1, &mut spares, input)
}

pub fn part_2(input: &Reactions) -> usize {
    let mut spares = vec![];
    let mut opf = ORE / hunt(1, &mut spares, input);
    let mut fuel = ORE / opf;
    spares.clear();
    while hunt(fuel + 1, &mut spares, input) < ORE {
        spares.clear();
        opf = hunt(fuel, &mut spares, input) / fuel;
        fuel = ORE / opf;
    }
    fuel
}
