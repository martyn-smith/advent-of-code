use regex::Regex;
use std::fs;
use std:: collections::HashSet;

#[derive(Debug)]
pub struct IngredientString {
    inputs: Vec<(usize, String)>,
    output: (usize, String),
}

impl IngredientString {
    fn new(l: &str) -> Self {
        let ingredient_srch = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
        let split_srch = Regex::new(r"^(.*) => (.*)$").unwrap();
        let split_match = split_srch.captures(l).unwrap();
        let (inputs, output) = (&split_match[1], &split_match[2]);
        let inputs: Vec<(usize, String)> = ingredient_srch
            .captures_iter(inputs)
            .map(|c| (usize::from_str_radix(&c[1], 10).unwrap(), c[2].to_string()))
            .collect();
        let output = ingredient_srch.captures(output).unwrap();
        let output = (usize::from_str_radix(&output[1], 10).unwrap(), output[2].to_string());
        IngredientString {
            inputs,
            output
        }
    }
}
// 42108 < ans < 58302258
fn hunt(input: &Vec<IngredientString>, target: &str, endpoint: &str) -> usize {
    /* we could just use the return type of .find() and assume None means we've found ore,
    but I'd prefer not to take the risk.

    Unfortunately this recursive approach fails to identify the minimum requirement,
    since, although the data structure is not cyclic, it is possible for multiple parents
    to share a child. e.g:

    "3 ORE => 2 A", "1 A => 1 B", "1 A => 1 C", "1 B, 1 C => 1 FUEL" => 3 ORE

    rules out integer accumulation

    "3 ORE => 2 A", "3 ORE => 2 B", "1 A, 1 B => FUEL" => 4 ORE

    rules out floating-point accumulation

    */
    println!("hunting for {}", target);
    if target != endpoint {
        let l = input.iter()
                     .find(|&i| i.output.1 == target).unwrap();
        l.inputs.iter()
            .map(|i| {
                let mut qty = i.0 * hunt(input, &i.1, endpoint); 
                qty = qty / l.output.0 + (qty % l.output.0 != 0) as usize; 
                //qty /= l.output.0 as f64;
                println!("{} {}", i.1, qty); 
                qty
            })
            .sum()
    } else {
        1
    }
}

pub fn get_input() -> Vec<IngredientString> {
    fs::read_to_string("../data/14_very_small.txt")
        .unwrap()
        .lines()
        .map(|l| IngredientString::new(l))
        .collect()
}

pub fn part_1(input: &Vec<IngredientString>) -> usize {
    hunt(input, "FUEL", "ORE")
}
