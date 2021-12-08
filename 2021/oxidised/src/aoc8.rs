///
/// Advent of Code day 8: seven-segment displays make a pretty graph
///

/*
 * Starts out as a bipartite graph with 7! (5040) possibilities (?)
 * Given we always have all 10 numbers present, a deterministic solution should be possible.
 *
 * n | 0 1 2 3 4 5 6 7 8 9
 * -----------------------
 * l | 6 2 5 5 4 5 6 3 7 6
 *
 * First, identify 1, 4, 7, 8 by length;
 * 9 can be idenfitied as containing only the chars in 4 and 7
 *   (i.e. 4 + 7 == 9)
 * x->a can be determined as the only char in 7 and not 1
 * x->b can be determined as occurring in 4 and 9 and not 7
 * x->d can be determined as occurring only in 4 and not 1 or 7
 * x->e cen be determined as occurring in 8 and not 9
 * 6 can be identified as containing a and d and being of length 6
 * 0 can be identified as not containing d and being of length 6
 * x->c can be determined as being in 1 and not 6.
 * x->f can be determined as being in 1 and not being c.
 * x->g is the remainder
 */

const NUMBERS: [&'static str; 10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf",
                                     "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];

pub type Display = (Vec<String>, Vec<String>);

fn get_1478(line: &Vec<String>) -> usize {
    line.iter()
        .filter(|&l| by_len(l).is_some())
        .count()
}

fn by_len(s: &str) -> Option<usize> {
    match s.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None
    }
}
/*
fn solve(line: &Vec<String>) -> HashMap<char, char> {
    let mut nums : [Option<&str>; 10] = [None; 10];
    for l in line.iter() {
        if let Some(i) = by_len(l[..]) {
            nums[i] = l[..];
    }
    let mut segments = HashMap::new(); <char, char>
    //solve a
    for c in nums[7].unwrap().chars() {
        if !nums[1].unwrap().contains(c) {
            segments.insert(c, 'a');
            break;
        }
    }
    //solve b
    for c in nums[4].unwrap().chars() {
        for d in 
    }

}
*/
fn get_line(l: &str) -> Display {
    let mut s = l.split('|');
    let samples = s.next().unwrap().trim().split(' ').map(|s| s.to_string());
    let values = s.next().unwrap().trim().split(' ').map(|s| s.to_string());
    (samples.collect(), values.collect())
}

pub fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    include_str!("../../data/8.txt")
                    .lines()
                    .map(|l| get_line(l))
                    .collect()
}

pub fn part_1(input: &Vec<Display>) -> usize {
    input
        .iter()
        .map(|l| get_1478(&l.1))
        .sum()
}
