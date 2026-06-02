#[cfg(test)]
mod test_s1827 {
    use leetcode::s1827_minimum_operations_to_make_the_array_increasing::min_operations;

    #[test]
    fn test_case_1() {
        assert_eq!(min_operations(vec![1, 1, 1]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_operations(vec![1, 5, 2, 4, 1]), 14);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_operations(vec![8]), 0);
    }
}
