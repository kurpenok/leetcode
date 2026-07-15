#[cfg(test)]
mod test_s2114 {
    use leetcode::s2114_maximum_number_of_words_found_in_sentences::most_words_found;

    #[test]
    fn test_case_1() {
        assert_eq!(
            most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ]),
            6
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            most_words_found(vec![
                "please wait".to_string(),
                "continue to fight".to_string(),
                "continue to win".to_string()
            ]),
            3
        );
    }
}
