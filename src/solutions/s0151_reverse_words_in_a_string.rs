pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            reverse_words("a good   example".to_string()),
            "example good a"
        );
    }
}
