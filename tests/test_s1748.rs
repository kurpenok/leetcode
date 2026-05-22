#[cfg(test)]
mod test_s1748 {
    use leetcode::s1748_sum_of_unique_elements::sum_of_unique;

    #[test]
    fn test_case_1() {
        assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
