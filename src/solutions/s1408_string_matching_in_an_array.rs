pub fn string_matching(words: Vec<String>) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .filter(|(i, word)| {
            words
                .iter()
                .enumerate()
                .any(|(j, other)| *i != j && other.contains(*word))
        })
        .map(|(_, word)| word.clone())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            string_matching(vec![
                "mass".to_string(),
                "as".to_string(),
                "hero".to_string(),
                "superhero".to_string()
            ]),
            ["as", "hero"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            string_matching(vec![
                "leetcode".to_string(),
                "et".to_string(),
                "code".to_string()
            ]),
            ["et", "code"]
        );
    }

    #[test]
    fn test_case_3() {
        let expected_result: Vec<String> = Vec::new();
        assert_eq!(
            string_matching(vec![
                "blue".to_string(),
                "green".to_string(),
                "bu".to_string()
            ]),
            expected_result
        );
    }
}
