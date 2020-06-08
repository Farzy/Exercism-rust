#![feature(test)]

extern crate test;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let mut n = num;
    let digits = ((num as f64).log10().floor()) as u32 + 1;

    while n != 0 {
        sum += (n % 10).pow(digits);
        n /= 10;
    }

    sum == num
}

// Compare performance to a popular solution
// See https://exercism.io/tracks/rust/exercises/armstrong-numbers/solutions/79d4f3931f6047de9d9642b57f7c5337
pub fn is_armstrong_number_iter(num: u32) -> bool {
    let digits = ((num as f64).log10() + 1.).floor() as u32;
    (0..digits)
        .map(|i| (num / 10u32.pow(i) % 10).pow(digits))
        .sum::<u32>()
        == num
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const MAX: u32 = 10_000_000;

    #[bench]
    fn bench_is_armstrong_number(b: &mut Bencher) {
        let max = test::black_box(MAX);

        b.iter(|| (0..max).fold(true, |old, new| old && is_armstrong_number(new)));
    }

    #[bench]
    fn bench_is_armstrong_number_iter(b: &mut Bencher) {
        let max = test::black_box(MAX);

        b.iter(|| (0..max).fold(true, |old, new| old && is_armstrong_number_iter(new)));
    }
}
