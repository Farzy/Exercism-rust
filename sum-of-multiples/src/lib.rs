use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();

    for factor in factors {
        let numbers = (1..)
            .map(|x| *factor * x)
            .take_while(|x| *x < limit)
            .collect::<HashSet<u32>>();
        multiples.extend(numbers);
    }
    multiples.iter().sum::<u32>()
}
