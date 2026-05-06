#[cfg(test)]
mod test_s1624 {
    use leetcode::s1624_largest_substring_between_two_equal_characters::max_length_between_equal_characters;

    #[test]
    fn test_case_1() {
        assert_eq!(max_length_between_equal_characters("aa".to_string()), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_length_between_equal_characters("abca".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_length_between_equal_characters("cbzxy".to_string()), -1);
    }
}
