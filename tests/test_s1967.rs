#[cfg(test)]
mod test_s1967 {
    use leetcode::s1967_number_of_strings_that_appear_as_substrings_in_word::num_of_strings;

    #[test]
    fn test_case_1() {
        assert_eq!(
            num_of_strings(
                vec![
                    "a".to_string(),
                    "abc".to_string(),
                    "bc".to_string(),
                    "d".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            num_of_strings(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aaaaabbbbb".to_string()
            ),
            2
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            num_of_strings(
                vec!["a".to_string(), "a".to_string(), "a".to_string()],
                "ab".to_string()
            ),
            3
        );
    }
}
