#[cfg(test)]
mod test_s1859 {
    use leetcode::s1859_sorting_the_sentence::sort_sentence;

    #[test]
    fn test_case_1() {
        assert_eq!(
            sort_sentence("is2 sentence4 This1 a3".to_string()),
            "This is a sentence"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            sort_sentence("Myself2 Me1 I4 and3".to_string()),
            "Me Myself and I"
        );
    }
}
