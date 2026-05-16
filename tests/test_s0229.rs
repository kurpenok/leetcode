#[cfg(test)]
mod test_s0229 {
    use leetcode::s0229_majority_element_ii::majority_element;

    #[test]
    fn test_case_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), [3]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(majority_element(vec![1]), [1]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(majority_element(vec![1, 2]), [1, 2]);
    }
}
