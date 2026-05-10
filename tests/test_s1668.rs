#[cfg(test)]
mod test_s1668 {
    use leetcode::s1668_maximum_repeating_substring::max_repeating;

    #[test]
    fn test_case_1() {
        assert_eq!(max_repeating("ababc".to_string(), "ab".to_string()), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_repeating("ababc".to_string(), "ba".to_string()), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_repeating("ababc".to_string(), "ac".to_string()), 0);
    }
}
