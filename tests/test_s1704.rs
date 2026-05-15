#[cfg(test)]
mod test_s1704 {
    use leetcode::s1704_determine_if_string_halves_are_alike::halves_are_alike;

    #[test]
    fn test_case_1() {
        assert_eq!(halves_are_alike("book".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(halves_are_alike("textbook".to_string()), false);
    }
}
