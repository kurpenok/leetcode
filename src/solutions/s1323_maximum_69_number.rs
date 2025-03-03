pub fn maximum69_number(num: i32) -> i32 {
    let mut num_digits: Vec<char> = num.to_string().chars().collect();

    for i in 0..num_digits.len() {
        if num_digits[i] == '6' {
            num_digits[i] = '9';
            return num_digits
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
        }
    }

    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum69_number(9669), 9969);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum69_number(9996), 9999);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(maximum69_number(9999), 9999);
    }
}
