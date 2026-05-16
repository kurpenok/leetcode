#[cfg(test)]
mod test_s1716 {
    use leetcode::s1716_calculate_money_in_leetcode_bank::total_money;

    #[test]
    fn test_case_1() {
        assert_eq!(total_money(4), 10);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(total_money(10), 37);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(total_money(20), 96);
    }
}
