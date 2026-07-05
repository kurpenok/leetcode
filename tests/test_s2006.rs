#[cfg(test)]
mod test_s2006 {
    use leetcode::s2006_count_number_of_pairs_with_absolute_difference_k::count_k_difference;

    #[test]
    fn test_case_1() {
        assert_eq!(count_k_difference(vec![1, 2, 2, 1], 1), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_k_difference(vec![1, 3], 3), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
