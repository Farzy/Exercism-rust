
fn is_prime(n: u32) -> bool {
    assert!(n > 0, "n must be a positif number");
    if n <= 2 {
        return true;
    }
    if (n % 2) == 0 {
        return false;
    }
    // Try dividing
    let mut divisor = 3;
    while divisor <= (n/2) {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut index = 0;
    let mut last_prime = 2;
    while index < n {
        for n in (last_prime+1)..std::u32::MAX {
            if is_prime(n) {
                last_prime = n;
                index += 1;
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
