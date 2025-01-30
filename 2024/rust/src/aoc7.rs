fn sums(mut component: Vec<usize>) -> Vec<usize> {
    match component.len() {
        1 => vec![component.pop().unwrap()],
        _ => {
            let c = component.pop().unwrap();
            sums(component.clone())
                .into_iter()
                .flat_map(|a| [a + c, a * c])
                //.flatten()
                .collect::<Vec<_>>()
        }
    }
}

fn concats(mut component: Vec<usize>) -> Vec<usize> {
    match component.len() {
        1 => vec![component.pop().unwrap()],
        _ => {
            let c = component.pop().unwrap();
            concats(component.clone())
                .into_iter()
                .flat_map(|a| [a + c, a * c, (a * 10_usize.pow(c.ilog(10) + 1)) + c])
                //.flatten()
                .collect::<Vec<_>>()
        }
    }
}

fn is_valid_sum(equation: &(usize, Vec<usize>)) -> bool {
    let tgt = equation.0;
    sums(equation.1.clone()).into_iter().any(|r| r == tgt)
}

fn is_valid_concat(equation: &(usize, Vec<usize>)) -> bool {
    let tgt = equation.0;
    concats(equation.1.clone()).into_iter().any(|r| r == tgt)
}

pub fn get_input() -> Vec<(usize, Vec<usize>)> {
    include_str!("../../data/7.txt")
        .lines()
        .map(|l| {
            let mut s = l.split(':');
            let tgt = s.next().unwrap().parse::<usize>().unwrap(); //.strip('\n')
            let components = s
                .next()
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (tgt, components)
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter(|&eq| is_valid_sum(eq))
        .map(|eq| eq.0)
        .sum::<usize>()
}

pub fn part_2(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter(|&eq| is_valid_concat(eq))
        .map(|eq| eq.0)
        .sum::<usize>()
}
