#[cfg(test)]
mod test_s1913 {
    use leetcode::s1913_maximum_product_difference_between_two_pairs::max_product_difference;

    #[test]
    fn test_case_1() {
        assert_eq!(max_product_difference(vec![5, 6, 2, 7, 4]), 34);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]), 64);
    }
}
