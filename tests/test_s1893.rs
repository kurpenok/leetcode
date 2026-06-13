#[cfg(test)]
mod test_s1893 {
    use leetcode::s1893_check_if_all_the_integers_in_a_range_are_covered::is_covered;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21), false);
    }
}
