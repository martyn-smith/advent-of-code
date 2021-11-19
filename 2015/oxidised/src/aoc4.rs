use md5::{Md5, Digest};

pub fn get_input() -> &'static str {
    "iwrupvqb"
}

pub fn part_1(prefix: &str) -> usize {
    let mut n = 0usize;
    loop {
        let cand = prefix.to_owned() + &n.to_string();
        let hash = Md5::digest(cand.as_bytes());
        // if n % 10_000 == 0 {
        //     println!("trying {:?}", hash);
        // }
        if hash[0..5] == b"00000"[0..5] {
            break;
        }
        n += 1;
    }
    n
}

pub fn part_2(prefix: &str) -> usize {
    let mut n = 0usize;
    loop {
        let cand = prefix.to_owned() + &n.to_string();
        let hash = Md5::digest(cand.as_bytes());
        // if n % 10_000 == 0 {
        //     println!("trying {:?}", hash);
        // }
        if hash[0..6] == b"000000"[0..6] {
            break;
        }
        n += 1;
    }
    n
}
