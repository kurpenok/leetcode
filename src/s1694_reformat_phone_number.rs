pub fn reformat_number(number: String) -> String {
    let digits = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    let mut i = 0;
    let digits_len = digits.len();
    let mut new_number = String::new();

    while i < digits_len {
        let remaining = digits_len - i;
        if remaining > 4 {
            new_number.push_str(&digits[i..i + 3]);
            new_number.push('-');
            i += 3;
        } else {
            match remaining {
                4 => {
                    new_number.push_str(&digits[i..i + 2]);
                    new_number.push('-');
                    new_number.push_str(&digits[i + 2..]);
                }
                _ => new_number.push_str(&digits[i..]),
            }
            break;
        }
    }

    new_number
}
