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

// This version is twice as fast as factors_full_iter
pub fn factors(n: u64) -> Vec<u64> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_factors_full_iter(b: &mut Bencher) {
        b.iter(|| factors_full_iter(93_819_012_551));
    }

    #[bench]
    fn bench_factors(b: &mut Bencher) {
        b.iter(|| factors(93_819_012_551));
    }
}
