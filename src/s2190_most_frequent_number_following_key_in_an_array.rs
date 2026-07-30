use std::collections::HashMap;

pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
    let mut counts = HashMap::new();

    for window in nums.windows(2) {
        if window[0] == key {
            let target = window[1];
            *counts.entry(target).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(target, _)| target)
        .unwrap_or(0)
}
