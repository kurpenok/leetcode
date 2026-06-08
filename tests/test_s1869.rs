#[cfg(test)]
mod test_s1869 {
    use leetcode::s1869_longer_contiguous_segments_of_ones_than_zeros::check_zero_ones;

    #[test]
    fn test_case_1() {
        assert_eq!(check_zero_ones("1101".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_zero_ones("111000".to_string()), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(check_zero_ones("110100010".to_string()), false);
    }
}
