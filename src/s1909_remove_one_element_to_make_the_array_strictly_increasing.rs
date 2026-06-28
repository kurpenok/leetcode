pub fn can_be_increasing(nums: Vec<i32>) -> bool {
    let mut modified = false;
    let mut prev = nums[0];

    for i in 1..nums.len() {
        if prev >= nums[i] {
            if modified {
                return false;
            }
            modified = true;
        }

        if i == 1 || nums[i - 2] < nums[i] {
            prev = nums[i];
        } else {
            prev = nums[i - 1];
        }
    }

    true
}
