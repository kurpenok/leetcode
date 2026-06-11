#[cfg(test)]
mod test_s1880 {
    use leetcode::s1880_check_if_word_equals_summation_of_two_words::is_sum_equal;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_sum_equal("aaa".to_string(), "a".to_string(), "aab".to_string()),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            is_sum_equal("aaa".to_string(), "a".to_string(), "aaaa".to_string()),
            true
        );
    }
}
