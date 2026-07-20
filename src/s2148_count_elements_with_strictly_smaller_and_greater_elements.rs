pub fn count_elements(nums: Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    nums.iter().filter(|&n| min < n && n < max).count() as i32
}
