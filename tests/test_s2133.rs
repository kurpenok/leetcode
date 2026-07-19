#[cfg(test)]
mod test_s2133 {
    use leetcode::s2133_check_if_every_row_and_column_contains_all_numbers::check_valid;

    #[test]
    fn test_case_1() {
        assert_eq!(
            check_valid(vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]]),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            check_valid(vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]]),
            false
        );
    }
}
