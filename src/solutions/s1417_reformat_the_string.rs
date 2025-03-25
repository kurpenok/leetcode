pub fn reformat(s: String) -> String {
    let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
    let symbols: Vec<char> = s.chars().filter(|c| !c.is_digit(10)).collect();

    if (digits.len() as i32 - symbols.len() as i32).abs() > 1 {
        return String::new();
    }

    let mut reformatted_s: String = String::new();

    if digits.len() < symbols.len() {
        reformatted_s.push(*symbols.last().unwrap());
    }

    for (c_1, c_2) in digits.iter().zip(symbols.iter()) {
        reformatted_s.push(*c_1);
        reformatted_s.push(*c_2);
    }

    if digits.len() > symbols.len() {
        reformatted_s.push(*digits.last().unwrap());
    }

    reformatted_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(reformat("a0b1c2".to_string()), "0a1b2c");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reformat("leetcode".to_string()), "");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(reformat("1229857369".to_string()), "");
    }
}
