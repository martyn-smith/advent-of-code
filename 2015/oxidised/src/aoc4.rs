///
/// Advent of Code day 4: Santa's first Cryptominer
///
/*
 * Sticking with a single threaded implementation for now, it seems speedy enough
 */
use md5::{Digest, Md5};

pub fn get_input() -> &'static str {
    "iwrupvqb"
}

pub fn part_1(prefix: &str) -> usize {
    let mut n = 0usize;
    loop {
        let cand = format!("{}{}", prefix, n);
        let hash = format!("{:x}", Md5::digest(cand.as_bytes()));
        // if n % 10_000 == 0 {
        //     println!("trying {} = {}", cand, hash);
        // }
        if hash[0..5] == "00000"[0..5] {
            break;
        }
        n += 1;
    }
    n
}

pub fn part_2(prefix: &str) -> usize {
    let mut n = 0usize;
    loop {
        let cand = format!("{}{}", prefix, n);
        let hash = format!("{:x}", Md5::digest(cand.as_bytes()));
        // if n % 10_000 == 0 {
        //     println!("trying {} = {}", cand, hash);
        // }
        if hash[0..6] == "000000"[0..6] {
            break;
        }
        n += 1;
    }
    n
}
