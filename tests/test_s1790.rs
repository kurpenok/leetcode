#[cfg(test)]
mod test_s1790 {
    use leetcode::s1790_check_if_one_string_swap_can_make_strings_equal::are_almost_equal;

    #[test]
    fn test_case_1() {
        assert_eq!(
            are_almost_equal("bank".to_string(), "kanb".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            are_almost_equal("attack".to_string(), "defend".to_string()),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            are_almost_equal("kelb".to_string(), "kelb".to_string()),
            true
        );
    }
}
