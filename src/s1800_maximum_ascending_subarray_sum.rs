pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut current_sum = nums[0];
    let mut max_sum = current_sum;

    for i in 1..nums.len() {
        if nums[i - 1] >= nums[i] {
            max_sum = max_sum.max(current_sum);
            current_sum = 0;
        }
        current_sum += nums[i];
    }

    max_sum.max(current_sum)
}
