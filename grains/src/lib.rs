pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s <= 64, "Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|n| square(n)).sum::<u64>()
}
