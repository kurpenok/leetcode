#[cfg(test)]
mod test_s1662 {
    use leetcode::s1662_check_if_two_string_arrays_are_equivalent::array_strings_are_equal;

    #[test]
    fn test_case_1() {
        assert_eq!(
            array_strings_are_equal(
                vec!["ab".to_string(), "c".to_string()],
                vec!["a".to_string(), "bc".to_string()]
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            array_strings_are_equal(
                vec!["a".to_string(), "cb".to_string()],
                vec!["ab".to_string(), "c".to_string()]
            ),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            array_strings_are_equal(
                vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
                vec!["abcddefg".to_string()]
            ),
            true
        );
    }
}
