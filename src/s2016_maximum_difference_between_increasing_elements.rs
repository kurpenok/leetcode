pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((-1, i32::MAX), |(max_diff, min_val), num| {
            if num > &min_val {
                (max_diff.max(num - min_val), min_val)
            } else {
                (max_diff, min_val.min(*num))
            }
        })
        .0
}
