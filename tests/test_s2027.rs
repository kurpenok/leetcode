#[cfg(test)]
mod test_s2027 {
    use leetcode::s2027_minimum_moves_to_convert_string::minimum_moves;

    #[test]
    fn test_case_1() {
        assert_eq!(minimum_moves("XXX".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(minimum_moves("XXOX".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(minimum_moves("OOOO".to_string()), 0);
    }
}
