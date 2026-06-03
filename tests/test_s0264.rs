#[cfg(test)]
mod test_s0264 {
    use leetcode::s0264_ugly_number_ii::nth_ugly_number;

    #[test]
    fn test_case_1() {
        assert_eq!(nth_ugly_number(10), 12);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(nth_ugly_number(1), 1);
    }
}
