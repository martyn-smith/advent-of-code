///
/// Advent of Code day 1: The Rocket Equation
///

//use std::fs;

fn get_fuel(module_mass: &usize) -> usize {
    (module_mass / 3).saturating_sub(2)
}

fn get_recursive_fuel(module_mass: &usize) -> usize {
    let mut fuel = get_fuel(module_mass);
    let mut m = get_fuel(&fuel);
    while m > 0 {
        fuel += m;
        m = get_fuel(&m);
    }
    fuel
}

pub fn get_input() -> Vec<usize> {
    let input = include_str!("../../data/1.txt");
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

pub fn part_1(masses: &[usize]) -> usize {
    masses.iter().map(get_fuel).sum()
}

pub fn part_2(masses: &[usize]) -> usize {
    masses.iter().map(get_recursive_fuel).sum()
}
