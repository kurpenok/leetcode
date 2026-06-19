#[cfg(test)]
mod test_s1903 {
    use leetcode::s1903_largest_odd_number_in_string::largest_odd_number;

    #[test]
    fn test_case_1() {
        assert_eq!(largest_odd_number("52".to_string()), "5");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(largest_odd_number("4206".to_string()), "");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(largest_odd_number("35427".to_string()), "35427");
    }
}
