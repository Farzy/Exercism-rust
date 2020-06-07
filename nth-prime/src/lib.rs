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
    while divisor <= ((n as f64).sqrt() as u32) {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

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

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(is_prime(11));
    }
}
