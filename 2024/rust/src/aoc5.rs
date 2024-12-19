use std::cmp::Ordering;
use std::collections::HashMap;

struct PageOrderError {
    rule: (usize, usize),
    violator: (usize, usize),
}

fn is_valid(job: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    (0..job.len() - 1).all(|i| {
        let page = job[i];
        if let Some(reqs) = rules.get(&page) {
            reqs.iter()
                .all(|r| job[..i].contains(r) || !job.contains(r))
        } else {
            true
        }
    })
}

pub fn get_input() -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut input = include_str!("../../data/5.txt").split("\n\n");
    let mut rules = HashMap::new();
    for l in input.next().unwrap().lines() {
        let mut s = l.split('|');
        let b = s.next().unwrap().parse::<usize>().unwrap();
        let t = s.next().unwrap().parse::<usize>().unwrap();
        rules
            .entry(t)
            .and_modify(|req: &mut Vec<usize>| req.push(b))
            .or_insert(vec![b]);
    }
    let jobs = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (rules, jobs)
}

pub fn part_1(input: &(HashMap<usize, Vec<usize>>, Vec<Vec<usize>>)) -> usize {
    let rules = &input.0;
    let jobs = &input.1;
    jobs.iter()
        .filter_map(|j| {
            if is_valid(j, rules) {
                Some(j[j.len() / 2])
            } else {
                None
            }
        })
        .sum::<usize>()
}

pub fn part_2(input: &(HashMap<usize, Vec<usize>>, Vec<Vec<usize>>)) -> usize {
    let rules = &input.0;
    let mut jobs = input.1.clone();
    jobs.iter_mut()
        .filter_map(|j| {
            if !is_valid(j, rules) {
                j.sort_by(|p1, p2| {
                    if let Some(req) = rules.get(p1) {
                        if req.contains(p2) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    } else {
                        Ordering::Less
                    }
                });
                Some(j[j.len() / 2])
            } else {
                None
            }
        })
        .sum::<usize>()
}
