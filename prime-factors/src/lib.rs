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
