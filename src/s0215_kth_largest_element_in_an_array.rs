pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums[nums.len() - k as usize]
}
