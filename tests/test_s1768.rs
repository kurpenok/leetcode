#[cfg(test)]
mod test_s1768 {
    use leetcode::s1768_merge_strings_alternately::merge_alternately;

    #[test]
    fn test_case_1() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }
}
