#![feature(test)]

extern crate test;

pub fn factors_full_iter(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut number = n;

    for factor in 2..=number {
        while number % factor == 0 {
            factors.push(factor);
            number /= factor;
        }
        if number == 1 {
            break;
        }
    }

    factors
}

// This version is twice as fast as factors_full_iter by avoiding factors multiple of 2
pub fn factors_no_even(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut number = n;

    let mut factor: u64 = 2;
    loop {
        while number % factor == 0 {
            factors.push(factor);
            number /= factor;
        }
        if number == 1 || factor > number {
            break;
        }
        match factor {
            2 => factor = 3,
            _ => factor += 2,
        }
    }

    factors
}

// This version is 2.15 times faster than factors_full_iter by avoiding factors multiple of 2 and 5
pub fn factors_no_even_5(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut number = n;

    let mut factor: u64 = 2;
    let mut factor_inc = [2, 2, 2, 4].iter().cycle();

    loop {
        while number % factor == 0 {
            factors.push(factor);
            number /= factor;
        }
        if number == 1 || factor > number {
            break;
        }
        match factor {
            2 => factor = 3,
            3 => factor = 5,
            5 => factor = 7,
            _ => factor += *factor_inc.next().unwrap(),
        }
    }

    factors
}

// This version is 3.3 to 3.4 times faster than factors_full_iter by avoiding factors multiple of 2, 3 and 5
pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut number = n;

    let mut factor: u64 = 2;
    // Iterator for 4 first prime numbers
    let factor_inc_once = [1, 2, 2, 4].iter();
    // Iterator for factors starting at 11, avoiding multiples of 2, 3 and 5
    let factor_inc_3_5 = [2, 4, 2, 4, 6, 2, 6, 4].iter().cycle();
    let mut factor_inc = factor_inc_once.chain(factor_inc_3_5);

    loop {
        while number % factor == 0 {
            factors.push(factor);
            number /= factor;
        }
        if number == 1 || factor > number {
            break;
        }
        factor += *factor_inc.next().unwrap();
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_factors_full_iter(b: &mut Bencher) {
        b.iter(|| factors_full_iter(93_819_012_551));
    }

    #[bench]
    fn bench_factors_no_even(b: &mut Bencher) {
        b.iter(|| factors_no_even(93_819_012_551));
    }

    #[bench]
    fn bench_factors_no_even_5(b: &mut Bencher) {
        b.iter(|| factors_no_even_5(93_819_012_551));
    }

    #[bench]
    fn bench_factors(b: &mut Bencher) {
        b.iter(|| factors(93_819_012_551));
    }
}
