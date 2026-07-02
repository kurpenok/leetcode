#[cfg(test)]
mod test_s1952 {
    use leetcode::s1952_three_divisors::is_three;

    #[test]
    fn test_case_1() {
        assert_eq!(is_three(2), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_three(4), true);
    }
}
