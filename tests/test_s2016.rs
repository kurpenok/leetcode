#[cfg(test)]
mod test_s2016 {
    use leetcode::s2016_maximum_difference_between_increasing_elements::maximum_difference;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum_difference(vec![7, 1, 5, 4]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum_difference(vec![9, 4, 3, 2]), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
