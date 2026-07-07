pub fn are_numbers_ascending(s: String) -> bool {
    s.split_whitespace()
        .filter_map(|word| {
            if word.chars().all(|c| c.is_numeric()) {
                word.parse::<i32>().ok()
            } else {
                None
            }
        })
        .collect::<Vec<i32>>()
        .windows(2)
        .all(|n| n[0] < n[1])
}
