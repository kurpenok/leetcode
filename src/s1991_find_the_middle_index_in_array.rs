pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().sum::<i32>();
    let mut left_sum = 0;

    nums.iter()
        .enumerate()
        .find(|&(_, n)| {
            let is_match = left_sum == sum - left_sum - n;
            left_sum += n;
            is_match
        })
        .map(|(index, _)| index as i32)
        .unwrap_or(-1)
}
