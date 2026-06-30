pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|&x| nums[x as usize]).collect()
}
