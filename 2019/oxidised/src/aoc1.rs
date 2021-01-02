fn get_fuel(module_mass: &usize) -> usize {
    (module_mass / 3).checked_sub(2).unwrap_or(0)
}

fn total_fuel(module_mass: &usize) -> usize {
    let mut fuel = get_fuel(module_mass);
    let mut m = get_fuel(&fuel);
    while m > 0 {
        fuel += m;
        m = get_fuel(&m);
    }
    fuel
}

pub fn get_total_mass(masses: &Vec<usize>) -> usize {
    masses.iter()
        .map(|i| get_fuel(i))
        .sum()
}

pub fn get_recursive_total_mass(masses: &Vec<usize>) -> usize {
    masses.iter()
        .map(|i| total_fuel(i))
        .sum()
}

