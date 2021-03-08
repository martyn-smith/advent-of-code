// use regex::Regex;
// use std::fs;
// use std:: collections::HashSet;

// #[derive(Hash)]
// pub struct IngredientString {
//     inputs: Vec<(usize, String)>,
//     output: (usize, String),
// }

// impl IngredientString {
//     fn new(l: &str) -> Self {
//         let ingredient_srch = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
//         let split_srch = Regex::new(r"^(.*) => (.*)$").unwrap();
//         let split_match = split_srch.captures(l).unwrap();
//         let (inp, outp) = (&split_match[1], &split_match[2]);
//         let inputs: Vec<(usize, String)> = ingredient_srch
//             .captures_iter(inp)
//             .map(|c| (usize::from_str_radix(&c[1], 10).unwrap(), c[2].to_string()))
//             .collect();
//         let outp = ingredient_srch.captures(outp).unwrap();
//         let output = (usize::from_str_radix(&outp[1], 10).unwrap(), outp[2].to_string());
//         IngredientString {
//             inputs,
//             output
//         }
//     }
// }

// fn hunt(input: &HashSet<IngredientString>, target: &str) -> usize {
//     let l = input.get(target);
//     l.inputs.iter().map(|i| hunt(input, i.1)).sum()
// }

// pub fn get_input() -> HashSet<IngredientString> {
//     let input = fs::read_to_string("../data/14.txt").unwrap();
//     input.lines().map(|l| IngredientString::new(l)).collect()
// }

// pub fn part_1(input: &HashSet<IngredientString>) -> usize {
//     hunt(input, "fuel")
// }
