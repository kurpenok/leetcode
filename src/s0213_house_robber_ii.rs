fn rob_linear(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    } else if n == 1 {
        return nums[0];
    }

    let mut prev_2 = nums[0];
    let mut prev_1 = nums[0].max(nums[1]);

    for i in 2..n {
        let current = prev_1.max(prev_2 + nums[i]);
        prev_2 = prev_1;
        prev_1 = current;
    }

    prev_1
}

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        rob_linear(&nums[..nums.len() - 1]).max(rob_linear(&nums[1..]))
    }
}
