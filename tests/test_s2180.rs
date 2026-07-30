#[cfg(test)]
mod test_s2180 {
    use leetcode::s2180_count_intergers_with_even_digit_sum::count_even;

    #[test]
    fn test_case_1() {
        assert_eq!(count_even(4), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_even(30), 14);
    }
}
