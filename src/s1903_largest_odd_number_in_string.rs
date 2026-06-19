pub fn largest_odd_number(num: String) -> String {
    for (i, c) in num.char_indices().rev() {
        if let Some(digit) = c.to_digit(10) {
            if digit % 2 != 0 {
                return num[..=i].to_string();
            }
        }
    }

    "".to_string()
}
