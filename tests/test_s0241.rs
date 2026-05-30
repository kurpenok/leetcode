#[cfg(test)]
mod test_s0241 {
    use leetcode::s0241_different_ways_to_add_parentheses::diff_ways_to_compute;

    #[test]
    fn test_case_1() {
        assert_eq!(diff_ways_to_compute("2-1-1".to_string()), vec![2, 0]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            diff_ways_to_compute("2*3-4*5".to_string()),
            vec![-34, -10, -14, -10, 10]
        );
    }
}
