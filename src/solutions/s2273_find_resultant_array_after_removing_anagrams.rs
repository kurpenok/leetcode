pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut new_words: Vec<String> = Vec::new();
    let mut prev_symbols = [0; 26];

    for word in words {
        let mut current_symbols = [0; 26];
        word.bytes()
            .for_each(|b| current_symbols[(b - b'a') as usize] += 1);
        if prev_symbols != current_symbols {
            new_words.push(word);
        }
        prev_symbols = current_symbols;
    }

    new_words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            remove_anagrams(vec![
                "abba".to_string(),
                "baba".to_string(),
                "bbaa".to_string(),
                "cd".to_string(),
                "cd".to_string(),
            ]),
            ["abba", "cd"],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            remove_anagrams(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ]),
            ["a", "b", "c", "d", "e"],
        );
    }
}
