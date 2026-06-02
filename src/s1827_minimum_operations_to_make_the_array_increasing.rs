pub fn min_operations(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut operations = 0;
    let mut prev = nums[0];

    for &n in nums.iter().skip(1) {
        if n <= prev {
            let needed = prev + 1;
            operations += needed - n;
            prev = needed;
        } else {
            prev = n;
        }
    }

    operations
}
