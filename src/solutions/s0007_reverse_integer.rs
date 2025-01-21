pub fn reverse(x: i32) -> i32 {
    let x_digits: Vec<char> = format!("{}", x).chars().collect();
    let mut reversed_x_digits: String = String::new();

    let mut non_zero_flag = false;
    for i in (0..x_digits.len()).rev() {
        if x_digits[i] == '-' || (x_digits[i] == '0' && !non_zero_flag) {
            continue;
        }
        reversed_x_digits.push(x_digits[i]);
        non_zero_flag = true;
    }

    match reversed_x_digits.parse::<i32>() {
        Ok(reversed_x) => {
            if x < 0 {
                -reversed_x
            } else {
                reversed_x
            }
        }
        Err(_) => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(reverse(120), 21);
    }
}
