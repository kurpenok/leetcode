#[cfg(test)]
mod test_s1941 {
    use leetcode::s1941_check_if_all_characters_have_equal_number_of_occurrences::are_occurrences_equal;

    #[test]
    fn test_case_1() {
        assert_eq!(are_occurrences_equal("abacbc".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(are_occurrences_equal("aaabb".to_string()), false);
    }
}
