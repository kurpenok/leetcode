#[cfg(test)]
mod test_s2176 {
    use leetcode::s2176_count_equal_and_divisible_pairs_in_an_array::count_pairs;

    #[test]
    fn test_case_1() {
        assert_eq!(count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
