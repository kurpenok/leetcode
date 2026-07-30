fn sum_digits_is_even(n: i32) -> bool {
    let mut n = n;
    let mut s = 0;

    while n != 0 {
        s += n % 10;
        n /= 10;
    }

    s % 2 == 0
}

pub fn count_even(num: i32) -> i32 {
    (1..=num).filter(|&n| sum_digits_is_even(n)).count() as i32
}
