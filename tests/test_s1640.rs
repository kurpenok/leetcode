#[cfg(test)]
mod test_s1640 {
    use leetcode::s1640_check_array_formation_through_concatenation::can_form_array;

    #[test]
    fn test_case_1() {
        assert_eq!(can_form_array(vec![15, 88], vec![vec![88], vec![15]]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            can_form_array(vec![49, 18, 16], vec![vec![16, 18, 49]]),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]]),
            true
        );
    }
}
