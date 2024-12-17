use std::collections::HashMap;

fn is_valid(job: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    (0..job.len() - 1).all(|i| is_satisfied(i, rules, &job))
}

fn is_satisfied(i: usize, rules: &HashMap<usize, Vec<usize>>, job: &Vec<usize>) -> bool {
    let n = job[i];
    if let Some(reqs) = rules.get(&n) {
        dbg!(&reqs);
        reqs.into_iter().all(|r| job[i..].contains(&r))
    } else {
        true
    }
}

pub fn get_input() -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut input = include_str!("../../data/5.small.txt").split("\n\n");
    let mut rules = HashMap::new();
    for l in input.next().unwrap().lines() {
        let mut s = l.split('|');
        let r = s.next().unwrap().parse::<usize>().unwrap();
        let t = s.next().unwrap().parse::<usize>().unwrap();
        rules
            .entry(r)
            .and_modify(|req: &mut Vec<usize>| req.push(t))
            .or_insert(vec![t]);
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
    dbg!(&input.1);
    input.1.iter().filter(|j| is_valid(j, &input.0)).count()
}
