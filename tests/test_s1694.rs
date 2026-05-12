#[cfg(test)]
mod test_s1694 {
    use leetcode::s1694_reformat_phone_number::reformat_number;

    #[test]
    fn test_case_1() {
        assert_eq!(reformat_number("1-23-45 6".to_string()), "123-456");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reformat_number("123 4-567".to_string()), "123-45-67");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(reformat_number("123 4-5678".to_string()), "123-456-78");
    }
}
