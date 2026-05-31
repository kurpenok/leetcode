#[cfg(test)]
mod test_s1816 {
    use leetcode::s1816_truncate_sentence::truncate_sentence;

    #[test]
    fn test_case_1() {
        assert_eq!(
            truncate_sentence("Hello how are you Contestant".to_string(), 4),
            "Hello how are you"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            truncate_sentence("What is the solution to this problem".to_string(), 4),
            "What is the solution"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            truncate_sentence("chopper is not a tanuki".to_string(), 5),
            "chopper is not a tanuki"
        );
    }
}
