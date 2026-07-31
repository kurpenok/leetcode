use std::collections::BTreeSet;

pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut indices = BTreeSet::new();

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if (i as i32 - j as i32).abs() <= k && nums[j] == key {
                indices.insert(i as i32);
            }
        }
    }

    indices.into_iter().collect()
}
