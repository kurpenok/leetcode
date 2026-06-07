pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut total = 0;

    for mask in 0..(1 << nums.len()) {
        let mut xor_sum = 0;
        for i in 0..nums.len() {
            if mask & (1 << i) != 0 {
                xor_sum ^= nums[i];
            }
        }
        total += xor_sum;
    }

    total
}
