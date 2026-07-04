pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut a = *nums.iter().min().unwrap();
    let mut b = *nums.iter().max().unwrap();

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}
