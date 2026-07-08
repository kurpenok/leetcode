#[cfg(test)]
mod test_s2068 {
    use leetcode::s2068_check_whether_two_strings_are_almost_equivalent::check_almost_equivalent;

    #[test]
    fn test_case_1() {
        assert_eq!(
            check_almost_equivalent("aaaa".to_string(), "bccb".to_string()),
            false
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            check_almost_equivalent("abcdeef".to_string(), "abaaacc".to_string()),
            true
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            check_almost_equivalent("cccddabba".to_string(), "babababab".to_string()),
            true
        );
    }
}
