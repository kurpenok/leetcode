#[cfg(test)]
mod test_s1844 {
    use leetcode::s1844_replace_all_digits_with_characters::replace_digits;

    #[test]
    fn test_case_1() {
        assert_eq!(replace_digits("a1c1e1".to_string()), "abcdef");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(replace_digits("a1b2c3d4e".to_string()), "abbdcfdhe");
    }
}
