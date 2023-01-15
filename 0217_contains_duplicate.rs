use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut used: HashMap<i32, bool> = HashMap::new();

        for n in nums {
            if used.contains_key(&n) {
                return true;
            } else {
                used.insert(n, true);
            }
        }

        false
    }
}
