pub fn total_money(n: i32) -> i32 {
    let full = n as i64 / 7;
    let rest = n as i64 % 7;

    let full_sum = 7 * full * (full + 7) / 2;
    let rest_sum = rest * (full + 1) + rest * (rest - 1) / 2;

    (full_sum + rest_sum) as i32
}
