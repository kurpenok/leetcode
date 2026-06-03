pub fn sum_base(n: i32, k: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut n = n.abs();
    let mut sum = 0;
    while n > 0 {
        sum += n % k;
        n /= k;
    }

    sum
}
