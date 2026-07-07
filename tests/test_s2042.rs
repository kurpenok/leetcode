#[cfg(test)]
mod test_s2042 {
    use leetcode::s2042_check_if_numbers_are_ascending_in_a_sentence::are_numbers_ascending;

    #[test]
    fn test_case_1() {
        assert_eq!(
            are_numbers_ascending(
                "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            are_numbers_ascending("hello world 5 x 5".to_string()),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            are_numbers_ascending(
                "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
            ),
            false
        );
    }
}
