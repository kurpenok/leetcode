#[cfg(test)]
mod test_s2160 {
    use leetcode::s2160_minimum_sum_of_four_digit_number_after_splitting_digits::minimum_sum;

    #[test]
    fn test_case_1() {
        assert_eq!(minimum_sum(2932), 52);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(minimum_sum(4009), 13);
    }
}
