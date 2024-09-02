pub fn add_strings(num1: String, num2: String) -> String {
    let num1_chars: Vec<char> = num1.chars().rev().collect();
    let num2_chars: Vec<char> = num2.chars().rev().collect();

    let mut result: String = String::new();
    let mut ten_indicator: i8 = 0;

    for i in 0..std::cmp::max(num1.len(), num2.len()) {
        let mut sum = ten_indicator;

        if i < num1.len() {
            sum += num1_chars[i] as i8 - '0' as i8;
        }

        if i < num2.len() {
            sum += num2_chars[i] as i8 - '0' as i8;
        }

        if sum >= 10 {
            ten_indicator = 1;
        } else {
            ten_indicator = 0;
        };

        sum %= 10;

        result = format!("{}{}", sum, result);
    }

    if ten_indicator > 0 {
        result = format!("{}{}", 1, result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            add_strings("11".to_string(), "123".to_string()),
            "134".to_string()
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            add_strings("456".to_string(), "77".to_string()),
            "533".to_string()
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            add_strings("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            add_strings("6913259244".to_string(), "71103343".to_string()),
            "6984362587".to_string()
        );
    }
}
