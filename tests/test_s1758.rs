#[cfg(test)]
mod test_s1758 {
    use leetcode::s1758_minimum_changes_to_make_alternating_binary_string::min_operations;

    #[test]
    fn test_case_1() {
        assert_eq!(min_operations("0100".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_operations("10".to_string()), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_operations("1111".to_string()), 2);
    }
}
