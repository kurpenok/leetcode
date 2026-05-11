#[cfg(test)]
mod test_s0003 {
    use leetcode::s0003_longest_substring_without_repeating_characters::length_of_longest_substring;

    #[test]
    fn test_case_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
