use std::{cmp::Reverse, collections::BinaryHeap};

pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut min_heap = BinaryHeap::with_capacity(k as usize + 1);

    for (i, &n) in nums.iter().enumerate() {
        min_heap.push(Reverse((n, i)));
        if min_heap.len() > k as usize {
            min_heap.pop();
        }
    }

    let mut result_pairs = min_heap
        .into_iter()
        .map(|Reverse(pair)| pair)
        .collect::<Vec<(i32, usize)>>();
    result_pairs.sort_by_key(|&(_, i)| i);
    result_pairs.into_iter().map(|(n, _)| n).collect()
}
