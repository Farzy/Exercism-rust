use std::collections::HashSet;

pub fn sum_of_multiples2(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();

    // Remove factor==0 otherwise our infinite iterator never ends
    for factor in factors.iter().filter(|x| **x != 0) {
        let numbers = (1..)
            .map(|x| *factor * x)
            .take_while(|x| *x < limit)
            .collect::<HashSet<u32>>();
        multiples.extend(numbers);
    }
    multiples.iter().sum::<u32>()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().any(|f| *f != 0 && *x % *f == 0))
        .sum::<u32>()
}
