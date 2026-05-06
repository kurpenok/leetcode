#[cfg(test)]
mod test_s1629 {
    use leetcode::s1629_slowest_key::slowest_key;

    #[test]
    fn test_case_1() {
        assert_eq!(slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()), 'c');
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
            'a'
        );
    }
}
