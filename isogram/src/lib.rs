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
