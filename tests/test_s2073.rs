#[cfg(test)]
mod test_s2073 {
    use leetcode::s2073_time_needed_to_buy_tickets::time_required_to_buy;

    #[test]
    fn test_case_1() {
        assert_eq!(time_required_to_buy(vec![2, 3, 2], 2), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
