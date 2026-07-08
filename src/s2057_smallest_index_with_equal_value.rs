pub fn smallest_equal(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .find(|&(i, &n)| i % 10 == n as usize)
        .map(|(i, _)| i as i32)
        .unwrap_or(-1)
}
