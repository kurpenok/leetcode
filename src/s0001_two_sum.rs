use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut differences_map: HashMap<i32, usize> = HashMap::new();

    for (j, &n) in nums.iter().enumerate() {
        if let Some(&i) = differences_map.get(&(target - n)) {
            return vec![i as i32, j as i32];
        }
        differences_map.insert(n, j);
    }

    unreachable!()
}
