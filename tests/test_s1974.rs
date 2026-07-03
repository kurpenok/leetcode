#[cfg(test)]
mod test_s1974 {
    use leetcode::s1974_minimum_time_to_type_word_using_special_typewriter::min_time_to_type;

    #[test]
    fn test_case_1() {
        assert_eq!(min_time_to_type("abc".to_string()), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_time_to_type("bza".to_string()), 7);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_time_to_type("zjpc".to_string()), 34);
    }
}
