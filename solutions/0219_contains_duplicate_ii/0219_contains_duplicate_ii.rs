use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut numbers: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            if numbers.contains_key(&nums[i]) {
                if (numbers[&nums[i]] as i32 - i as i32).abs() <= k {
                    return true;
                } else {
                    numbers.insert(nums[i], i);
                }
            } else {
                numbers.insert(nums[i], i);
            }
        }
        false
    }
}
