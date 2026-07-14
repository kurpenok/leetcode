#[cfg(test)]
mod test_s2099 {
    use leetcode::s2099_find_subsequence_of_length_k_with_the_largest_sum::max_subsequence;

    #[test]
    fn test_case_1() {
        assert_eq!(max_subsequence(vec![2, 1, 3, 3], 2), [3, 3]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_subsequence(vec![-1, -2, 3, 4], 3), [-1, 3, 4]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_subsequence(vec![3, 4, 3, 3], 2), [4, 3]);
    }
}
