use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = (1..=(num / 2))
        .filter(|&i| num % i == 0)
        .sum::<u64>();
    Some(match sum.cmp(&num) {
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    })
}
