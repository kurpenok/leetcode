#[cfg(test)]
mod test_s1672 {
    use leetcode::s1672_richest_customer_wealth::maximum_wealth;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }
}
