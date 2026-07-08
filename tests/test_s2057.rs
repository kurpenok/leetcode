#[cfg(test)]
mod test_s2057 {
    use leetcode::s2057_smallest_index_with_equal_value::smallest_equal;

    #[test]
    fn test_case_1() {
        assert_eq!(smallest_equal(vec![0, 1, 2]), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(smallest_equal(vec![4, 3, 2, 1]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), -1);
    }
}
