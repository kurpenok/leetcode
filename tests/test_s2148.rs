#[cfg(test)]
mod test_s2148 {
    use leetcode::s2148_count_elements_with_strictly_smaller_and_greater_elements::count_elements;

    #[test]
    fn test_case_1() {
        assert_eq!(count_elements(vec![11, 7, 2, 15]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_elements(vec![-3, 3, 3, 90]), 2);
    }
}
