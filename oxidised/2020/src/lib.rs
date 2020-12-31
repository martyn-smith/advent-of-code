mod aoc1;



#[cfg(test)]
mod tests {
    use std::fs;
    use super::aoc1;

    #[test]
    fn day_1() {
        let input = fs::read_to_string("data/1.txt").unwrap();
        let target = 2020;
        let entries = input.lines()
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect();
        assert_eq!(224436, aoc1::find_pair(&target, &entries).unwrap());   
        assert_eq!(303394260, aoc1::find_triple(&target, &entries).unwrap());
    }
    

}
