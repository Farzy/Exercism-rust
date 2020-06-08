#![feature(test)]

extern crate test;

fn is_prime_1(n: u32) -> bool {
    assert!(n > 0, "n must be a positif number");
    if n <= 2 {
        return true;
    }
    if (n % 2) == 0 {
        return false;
    }
    // Try dividing by every odd number >= 3 until we reach square root of n
    let mut divisor = 3;
    while divisor <= ((n as f64).sqrt() as u32) {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

// Faster version that avoid dividing by multiples of 2, 3 and 5
fn is_prime(n: u32) -> bool {
    assert!(n > 0, "n must be a positif number");
    if n <= 2 {
        return true;
    }
    if (n % 2) == 0 {
        return false;
    }
    // Try dividing by every odd number >= 3 until we reach square root of n
    let mut divisor = 3;
    // Iterator for 3 first prime numbers after 2
    let divisor_inc_once = [2, 2, 4].iter();
    // Iterator for divisors starting at 11, avoiding multiples of 2, 3 and 5
    let divisor_inc_3_5 = [2, 4, 2, 4, 6, 2, 6, 4].iter().cycle();
    let mut divisor_inc = divisor_inc_once.chain(divisor_inc_3_5);
    while divisor <= ((n as f64).sqrt() as u32) {
        if n % divisor == 0 {
            return false;
        }
        divisor += *divisor_inc.next().unwrap();
    }
    true
}

pub fn nth_1(n: u32) -> u32 {
    let mut last_prime = 2;
    for _ in 0..n {
        for candidate in (last_prime + 1)..std::u32::MAX {
            if is_prime_1(candidate) {
                last_prime = candidate;
                break;
            }
        }
    }
    last_prime
}

// Use faster primer test
pub fn nth(n: u32) -> u32 {
    let mut last_prime = 2;
    for _ in 0..n {
        for candidate in (last_prime + 1)..std::u32::MAX {
            if is_prime(candidate) {
                last_prime = candidate;
                break;
            }
        }
    }
    last_prime
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(104_743));
    }

    #[test]
    fn test_is_prime_1() {
        assert!(is_prime_1(2));
        assert!(is_prime_1(3));
        assert!(!is_prime_1(4));
        assert!(is_prime_1(5));
        assert!(!is_prime_1(6));
        assert!(is_prime_1(7));
        assert!(is_prime_1(11));
        assert!(is_prime_1(104_743));
    }

    #[bench]
    fn bench_nth_1(b: &mut Bencher) {
        b.iter(|| nth_1(10_000));
    }

    #[bench]
    fn bench_nth(b: &mut Bencher) {
        b.iter(|| nth(10_000));
    }
}
