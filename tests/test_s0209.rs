#[cfg(test)]
mod test_s0209 {
    use leetcode::s0209_minimum_size_subarray_sum::min_sub_array_len;

    #[test]
    fn test_case_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}
