use std::collections::HashMap;

pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut pairs: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        pairs.entry(n).or_default().push(i);
    }

    let mut pairs_counter = 0;

    for indices in pairs.values() {
        for (pos, &i) in indices.iter().enumerate() {
            for &j in indices.iter().skip(pos + 1) {
                if (i * j) as i32 % k == 0 {
                    pairs_counter += 1;
                }
            }
        }
    }

    pairs_counter
}

