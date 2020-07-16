use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // Convert to lowercase and to array, remove '-' and ' '
    let mut clean_candidate = candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c != ' ' && c != '-')
        .collect::<Vec<_>>();
    // Sort quickly
    clean_candidate.sort_unstable();
    // Find total length
    let len_total = clean_candidate.len();
    // Remove duplicates
    clean_candidate.dedup();
    // Compare lengths
    len_total == clean_candidate.len()
}

// Compare to https://exercism.io/tracks/rust/exercises/isogram/solutions/f53f6aebb04f4e7ca751a5e5075bc2ef
pub fn check_hansrodtang(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}
