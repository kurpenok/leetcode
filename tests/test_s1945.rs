#[cfg(test)]
mod test_s1945 {
    use leetcode::s1945_sum_of_digits_of_string_after_convert::get_lucky;

    #[test]
    fn test_case_1() {
        assert_eq!(get_lucky("iiii".to_string(), 1), 36);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(get_lucky("leetcode".to_string(), 2), 6);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(get_lucky("zbax".to_string(), 2), 8);
    }
}
