use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut values: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            if values.contains_key(&num) {
                values.insert(num, values[&num] + 1);
            } else {
                values.insert(num, 1);
            }
            println!("{}", &values[&num]);
        }

        let mut major: i32 = 0;
        let mut count: i32 = 0;
        for (key, value) in &values {
            if count < *value {
                count = *value;
                major = *key;
            }
        }

        return major;
    }
}
