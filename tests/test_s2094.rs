#[cfg(test)]
mod test_s2094 {
    use leetcode::s2094_finding_3_digit_even_numbers::find_even_numbers;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_even_numbers(vec![2, 1, 3, 0]),
            [102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_even_numbers(vec![2, 2, 8, 8, 2]),
            [222, 228, 282, 288, 822, 828, 882]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_even_numbers(vec![3, 7, 5]), []);
    }
}
