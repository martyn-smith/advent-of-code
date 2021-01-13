use std::fs;

pub fn part_1(orbits: &Vec<Vec<String>>) -> usize {
    // 
    // Identifies the total number of orbits (direct and indirect) specified in a 
    // text file with the following spec:

    // Each line has the form:
    
    // PLAN)SAT

    // where "PLAN" is the planet and "SAT" the satellite.
    // 
    let mut orbits = orbits.clone();
    let mut num_orbits = 0;
    while orbits.len() > 0 {
        num_orbits += orbits.len();
        let mut new_orbits: Vec<Vec<String>> = vec![];
        for o in &orbits {
            let satellite = &o[1];
            for d in &orbits {
                let planet = &d[0];
                if satellite == planet {
                    new_orbits.push(d.to_vec());
                }
            }
        }
        orbits = new_orbits
    }
    num_orbits
}

pub fn get_input() -> Vec<Vec<String>> {
    let input = fs::read_to_string("../data/6.txt").unwrap();
    //map into pairs
    input.lines().map(|l| l.split(')').map(|s| s.to_string()).collect()).collect()
}