#[cfg(test)]
mod test_s1800 {
    use leetcode::s1800_maximum_ascending_subarray_sum::max_ascending_sum;

    #[test]
    fn test_case_1() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(max_ascending_sum(vec![3, 6, 10, 1, 8, 9, 9, 8, 9]), 19);
    }
}
