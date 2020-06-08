#![feature(test)]

extern crate test;

pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s <= 64, "Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|n| square(n)).sum::<u64>()
}

pub fn total_no_iter() -> u64 {
    let mut sum = 0u64;
    for n in 1..=64 {
        sum += square(n as u32);
    }
    sum
}

pub fn total_no_iter_no_funccall() -> u64 {
    let mut sum = 0u64;
    for n in 0..64 {
        sum += 2u64.pow(n);
    }
    sum
}

/*

The performance of the 3 versions are really very close:

    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/grains-a371e5e07fde6a17

running 3 tests
test tests::bench_total                     ... bench:         225 ns/iter (+/- 104)
test tests::bench_total_no_iter             ... bench:         224 ns/iter (+/- 22)
test tests::bench_total_no_iter_no_funccall ... bench:         225 ns/iter (+/- 25)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out

    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/grains-a371e5e07fde6a17

running 3 tests
test tests::bench_total                     ... bench:         226 ns/iter (+/- 63)
test tests::bench_total_no_iter             ... bench:         221 ns/iter (+/- 23)
test tests::bench_total_no_iter_no_funccall ... bench:         225 ns/iter (+/- 48)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out

    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/grains-a371e5e07fde6a17

running 3 tests
test tests::bench_total                     ... bench:         221 ns/iter (+/- 31)
test tests::bench_total_no_iter             ... bench:         224 ns/iter (+/- 16)
test tests::bench_total_no_iter_no_funccall ... bench:         222 ns/iter (+/- 42)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out

 */

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_total(b: &mut Bencher) {
        b.iter(|| total());
    }

    #[bench]
    fn bench_total_no_iter(b: &mut Bencher) {
        b.iter(|| total_no_iter());
    }

    #[bench]
    fn bench_total_no_iter_no_funccall(b: &mut Bencher) {
        b.iter(|| total_no_iter());
    }
}
