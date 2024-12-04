pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut gcd = String::new();

    for i in 1..=str1.len() {
        let prefix = &str1[..i];
        let repeat_count_1 = str1.len() / prefix.len();
        let repeat_count_2 = str2.len() / prefix.len();
        if str1 == prefix.repeat(repeat_count_1) && str2 == prefix.repeat(repeat_count_2) {
            gcd = prefix.to_string();
        }
    }

    gcd
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC",
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB",
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(gcd_of_strings("LEET".to_string(), "CODE".to_string()), "",);
    }
}
