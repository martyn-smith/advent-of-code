use std::collections::HashMap;
use std::collections::VecDeque;
//use std::fs;

pub type Menu = HashMap<String, (isize, Vec<Chemical>)>;

#[derive(Debug)]
pub struct Chemical {
    name: String,
    qty: isize
}

fn get_order_and_spares(required_qty: isize, batch_size: isize) -> (isize, isize) {
    let n = required_qty / batch_size;
    let remainder = required_qty % batch_size;
    if remainder != 0 {
        (n + 1, remainder - batch_size)
    } else {
        (n, 0)
    }
}

// correct answer (31) for v_small input. Look for 165 on small.
// 42108 < ans < 843541
fn hunt(target: Chemical, recipes: &Menu) -> usize {
    //we do need to collate.
    let mut current = VecDeque::new();
    current.push_front(target);
    let mut ore = 0;
    while current.iter().any(|c| c.qty > 0) {
        println!("{:?}", current);
        let mut requirement = current.pop_front().unwrap();
        if requirement.qty < 0 {
            for chem in current.iter_mut() {
                if chem.name == requirement.name {
                    requirement.qty += chem.qty;
                    chem.qty = 0;
                }
            }
            current.push_back(requirement);
            continue;
        } else if requirement.qty == 0 {
            continue;
        } else {
            let (batch_size, next) = recipes.get(&requirement.name).unwrap();
            //it is not specified but recipes that require ORE, require only ORE
            let (num_orders, spare) = get_order_and_spares(requirement.qty, *batch_size);
            if spare < 0 {
                current.push_back(Chemical::new(&requirement.name[..], spare));
            }
            for n in next.into_iter() {
                if &n.name == "ORE" {
                    ore += n.qty * num_orders;
                } else {
                    current.push_back(Chemical::new(&n.name[..], n.qty * num_orders));
                }
            }
        }
    }
    for c in current.into_iter() {
        let excess = -1 * c.qty;
        ore -= excess / recipes.get(&c.name).unwrap().0;
    }
    ore as usize
}

impl Chemical {

    fn new(name: &str, qty: isize) -> Self {
        Chemical {
            name: name.to_string(),
            qty: qty
        }
    }

    fn from_str(s: &str) -> Self {
        let mut s = s.trim().split(' ');
        let qty = s.next().unwrap().parse::<isize>().unwrap();
        let name = s.next().unwrap().to_string();
        Chemical {
            name,
            qty
        }
    }
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
