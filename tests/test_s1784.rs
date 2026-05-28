#[cfg(test)]
mod test_s1784 {
    use leetcode::s1784_check_if_binary_string_has_at_most_one_segment_of_ones::check_ones_segment;

    #[test]
    fn test_case_1() {
        assert_eq!(check_ones_segment("1001".to_string()), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_ones_segment("110".to_string()), true);
    }
}
