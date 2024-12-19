use std::collections::LinkedList;

fn blink(stones: &mut LinkedList<usize>) {
    let mut to_split = vec![];
    for (i, val) in stones.iter_mut().enumerate() {
        if *val == 0 {
            *val = 1;
        } else if val.ilog10() % 2 == 1 {
            to_split.push(i);
        } else {
            *val = *val * 2024;
        }
    }
    for i in to_split.into_iter() {
        let mut back = stones.split_off(i);
        let val = back.pop_front().unwrap();
        dbg!(&val);
        let a = val / val.ilog10() as usize;
        let b = val % val.ilog10() as usize;
        stones.push_back(a);
        back.push_front(b);
        stones.append(&mut back);
    }
}

pub fn get_input() -> LinkedList<usize> {
    LinkedList::from([4610211usize, 4, 0, 59, 3907, 201586, 929, 33750])
}

pub fn part_1(input: &LinkedList<usize>) -> usize {
    let mut stones = input.clone();
    for _ in 1..25 {
        blink(&mut stones);
    }
    stones.len()
}
