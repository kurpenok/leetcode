use std::collections::HashMap;

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut counter = 0;
    let mut diffs = HashMap::new();

    for n in &nums {
        counter += *diffs.get(&(n + k)).unwrap_or(&0);
        counter += *diffs.get(&(n - k)).unwrap_or(&0);
        *diffs.entry(*n).or_insert(0) += 1;
    }

    counter
}
