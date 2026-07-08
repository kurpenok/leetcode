#[cfg(test)]
mod test_s2053 {
    use leetcode::s2053_kth_distinct_string_in_an_array::kth_distinct;

    #[test]
    fn test_case_1() {
        assert_eq!(
            kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            ),
            "a"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            kth_distinct(
                vec!["aaa".to_string(), "aa".to_string(), "a".to_string()],
                1
            ),
            "aaa"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3),
            ""
        );
    }
}
