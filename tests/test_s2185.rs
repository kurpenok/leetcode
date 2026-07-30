#[cfg(test)]
mod test_s2185 {
    use leetcode::s2185_counting_words_with_a_given_prefix::prefix_count;

    #[test]
    fn test_case_1() {
        assert_eq!(
            prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            ),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            prefix_count(
                vec![
                    "leetcode".to_string(),
                    "win".to_string(),
                    "loops".to_string(),
                    "success".to_string()
                ],
                "code".to_string()
            ),
            0
        );
    }
}
