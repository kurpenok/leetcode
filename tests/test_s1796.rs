#[cfg(test)]
mod test_s1796 {
    use leetcode::s1796_second_largest_digit_in_a_string::second_highest;

    #[test]
    fn test_case_1() {
        assert_eq!(second_highest("dfa12321afd".to_string()), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(second_highest("abc1111".to_string()), -1);
    }
}
