#![feature(test)]

extern crate test;

pub fn factors(n: u64) -> Vec<u64> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_factors(b: &mut Bencher) {
        b.iter(|| factors(93_819_012_551));
    }
}
