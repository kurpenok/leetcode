pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    nums[nums.len() - 1] * nums[nums.len() - 2] - nums[1] * nums[0]
}
