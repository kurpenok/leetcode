pub fn second_highest(s: String) -> i32 {
    let mut digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

    digits.sort_unstable_by(|a, b| b.cmp(a));
    digits.dedup();

    if digits.len() >= 2 {
        digits[1].to_digit(10).unwrap() as i32
    } else {
        -1
    }
}
