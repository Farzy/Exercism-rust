pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    let mut multiples: Vec<u32> = Vec::new();

    for factor in factors {
        let mut numbers = (1..)
            .map(|x| *factor * x)
            .take_while(|x| *x < limit)
            .collect::<Vec<u32>>();
        multiples.append(&mut numbers);
    }
    println!("For factor {:?} we found numbers {:?}", factors, multiples);
    sum
}
