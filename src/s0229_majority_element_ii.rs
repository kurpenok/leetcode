use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut majors = Vec::new();
    let mut counter = HashMap::new();

    for n in &nums {
        counter
            .entry(n)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (n, count) in counter {
        if count > nums.len() / 3 {
            majors.push(*n);
        }
    }

    majors.sort();

    majors
}
