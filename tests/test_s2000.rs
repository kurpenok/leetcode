#[cfg(test)]
mod test_s2000 {
    use leetcode::s2000_reverse_prefix_of_word::reverse_prefix;

    #[test]
    fn test_case_1() {
        assert_eq!(reverse_prefix("abcdefd".to_string(), 'd'), "dcbaefd");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reverse_prefix("xyxzxe".to_string(), 'z'), "zxyxxe");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(reverse_prefix("abcd".to_string(), 'z'), "abcd");
    }
}
