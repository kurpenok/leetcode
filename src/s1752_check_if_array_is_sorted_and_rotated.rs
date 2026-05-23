pub fn check(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n <= 1 {
        return true;
    }

    let mut count = 0;
    for i in 0..n - 1 {
        if nums[i] > nums[i + 1] {
            count += 1;
        }
    }

    if count == 0 {
        true
    } else if count == 1 {
        nums[n - 1] <= nums[0]
    } else {
        false
    }
}
