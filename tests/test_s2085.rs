#[cfg(test)]
mod test_s2085 {
    use leetcode::s2085_count_common_words_with_one_occurrence::count_words;

    #[test]
    fn test_case_1() {
        assert_eq!(
            count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string()
                ],
                vec![
                    "amazing".to_string(),
                    "leetcode".to_string(),
                    "is".to_string()
                ]
            ),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            count_words(
                vec!["b".to_string(), "bb".to_string(), "bbb".to_string()],
                vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
            ),
            0
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            count_words(
                vec!["a".to_string(), "ab".to_string()],
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "ab".to_string()
                ]
            ),
            1
        );
    }
}
