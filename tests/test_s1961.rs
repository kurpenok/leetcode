#[cfg(test)]
mod test_s1961 {
    use leetcode::s1961_check_if_string_is_a_prefix_of_array::is_prefix_string;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "apples".to_string()
                ]
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "apples".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string()
                ]
            ),
            false
        );
    }
}
