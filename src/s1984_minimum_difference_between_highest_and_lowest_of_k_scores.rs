pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut min_diff = i32::MAX;
    for slice in nums.windows(k as usize) {
        min_diff = min_diff.min(slice.last().unwrap() - slice.first().unwrap());
    }

    min_diff
}
