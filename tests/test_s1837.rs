#[cfg(test)]
mod test_s1837 {
    use leetcode::s1837_sum_of_digits_in_base_k::sum_base;

    #[test]
    fn test_case_1() {
        assert_eq!(sum_base(34, 6), 9);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sum_base(10, 10), 1);
    }
}
