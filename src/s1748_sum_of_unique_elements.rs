use std::collections::HashMap;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for &n in &nums {
        *counter.entry(n).or_insert(0) += 1;
    }

    counter
        .into_iter()
        .filter_map(|(n, count)| (count == 1).then_some(n))
        .sum()
}
