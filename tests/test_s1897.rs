#[cfg(test)]
mod test_s1897 {
    use leetcode::s1897_redistribute_characters_to_make_all_strings_equal::make_equal;

    #[test]
    fn test_case_1() {
        assert_eq!(
            make_equal(vec![
                "abc".to_string(),
                "aabc".to_string(),
                "bc".to_string()
            ]),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(make_equal(vec!["ab".to_string(), "b".to_string()]), false);
    }
}
