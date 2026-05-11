#[cfg(test)]
mod test_s0004 {
    use leetcode::s0004_median_of_two_sorted_arrays::find_median_sorted_arrays;

    #[test]
    fn test_case_1() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(find_median_sorted_arrays(vec![3], vec![-2, -1]), -1.0);
    }
}
