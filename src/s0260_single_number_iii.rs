pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let total_xor = nums.iter().fold(0, |acc, &n| acc ^ n);

    let rightmost_set_bit = total_xor & (-total_xor);

    let mut x = 0;
    let mut y = 0;
    for &n in &nums {
        if n & rightmost_set_bit != 0 {
            x ^= n;
        } else {
            y ^= n;
        }
    }

    vec![x, y]
}
