#[cfg(test)]
mod test_s1684 {
    use leetcode::s1684_count_the_number_of_consistent_strings::count_consistent_strings;

    #[test]
    fn test_case_1() {
        assert_eq!(
            count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            ),
            7
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string()
                ]
            ),
            4
        );
    }
}
