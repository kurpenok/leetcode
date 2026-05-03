use std::{cmp::Reverse, collections::HashMap};

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let frequencies: HashMap<i32, usize> = nums.iter().fold(HashMap::new(), |mut map, n| {
        let _ = *map.entry(*n).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    let mut nums = nums;
    nums.sort_unstable_by_key(|&n| (frequencies[&n], Reverse(n)));

    nums
}
