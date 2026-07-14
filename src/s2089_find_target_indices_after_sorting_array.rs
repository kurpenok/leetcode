pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (smaller_count, target_count) = nums.iter().fold((0, 0), |(smaller, equal), &n| {
        if n < target {
            (smaller + 1, equal)
        } else if n == target {
            (smaller, equal + 1)
        } else {
            (smaller, equal)
        }
    });
    (smaller_count..smaller_count + target_count).collect()
}
