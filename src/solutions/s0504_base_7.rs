pub fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let flag = if num < 0 { true } else { false };
    let mut num = num.abs();

    let mut result: String = String::new();

    while num > 0 {
        result = format!("{}{}", num % 7, result);
        num /= 7;
    }

    if flag {
        result = format!("-{}", result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(convert_to_base7(100), "202");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(convert_to_base7(-7), "-10");
    }
}
