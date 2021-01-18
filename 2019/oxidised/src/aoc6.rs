//use std::collections::HashMap;
use std::fs;

pub fn get_input() -> Vec<(String, String)> {
    let input = fs::read_to_string("../data/6.txt").unwrap();
    //map into pairs
    input
        .lines()
        .map(|l| {
            let o: Vec<String> = l.split(')').map(|s| s.to_string()).collect();
            (o[0].clone(), o[1].clone())
        })
        .collect()
}

pub fn part_1(orbits: &Vec<(String, String)>) -> usize {
    let mut orbits: Vec<Vec<String>> = orbits
        .iter()
        .map(|l| vec![l.0.clone(), l.1.clone()])
        .collect();
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

pub fn part_2(orbits: &Vec<(String, String)>) -> Option<usize> {
    let mut start = vec!["YOU"];
    let mut end = vec!["SAN"];
    // loop {
    //     if let Some(i) = end.index(start.last()).unwrap() {
    //         return Some(i + start.len());
    //     } else {
    //         start.push(orbits[orbits.index(start.last().unwrap())]);
    //         end.push(orbits[orbits.index(end.last().unwrap())]);
    //     }
    // }
    None
}
