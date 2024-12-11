use std::cmp::min;

fn get_divisors(mut n: usize, primes: &mut Vec<usize>) -> Vec<usize> {
    let _t = n;
    let mut factors = vec![1usize];
    while n > 1 && !primes.contains(&n) {
        if let Some(f) = get_divisor(n, primes) {
            factors.push(f);
            n /= f;
            factors.push(n);
        } else {
            primes.push(n);
            break;
        }
    }
    factors.push(_t);
    factors.sort();
    factors.dedup();
    //if _t == TEST_NUM {
    //    dbg!(_t, &primes, &factors);
    //}
    factors
}

fn p(n: usize, primes: &mut Vec<usize>) -> usize {
    let factors = get_divisors(n, primes);
    10 * factors.into_iter().sum::<usize>()
}

fn get_divisor(n: usize, primes: &[usize]) -> Option<usize> {
    primes.iter().rev().filter(|&&p| n % p == 0).copied().next()
}

fn unbounded_sieve(m: usize) -> Vec<usize> {
    let mut sieve = vec![0; m];
    for i in 1..m {
        for j in 1..=((m - 1) / i) {
            sieve[i * j] += 10 * i;
        }
    }
    sieve
}

fn bounded_sieve(m: usize) -> Vec<usize> {
    let mut sieve = vec![0; m];
    for i in 1..m {
        for j in 1..=min(50, (m - 1) / i) {
            //omitted as couldn't get the offsets to work.
            sieve[i * j] += 11 * i;
        }
    }
    sieve
}

pub fn get_input() -> usize {
    29_000_000
}

pub fn part_1(input: usize) -> usize {
    let presents = unbounded_sieve(input);
    presents
        .into_iter()
        .enumerate()
        .find_map(|(i, n)| if n >= input { Some(i) } else { None })
        .unwrap()
}

pub fn part_2(input: usize) -> usize {
    let presents = bounded_sieve(input);
    presents
        .into_iter()
        .enumerate()
        .find_map(|(i, n)| if n >= input { Some(i) } else { None })
        .unwrap()
}
