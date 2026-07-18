#[cfg(test)]
mod test_s2119 {
    use leetcode::s2119_a_number_after_a_double_reversal::is_same_after_reversals;

    #[test]
    fn test_case_1() {
        assert_eq!(is_same_after_reversals(526), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_same_after_reversals(1800), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_same_after_reversals(0), true);
    }
}
