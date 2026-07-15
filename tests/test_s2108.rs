#[cfg(test)]
mod test_s2108 {
    use leetcode::s2108_find_first_palindromic_string_in_the_array::first_palindrome;

    #[test]
    fn test_case_1() {
        assert_eq!(
            first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ]),
            "ada"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
            "racecar"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
            ""
        );
    }
}
