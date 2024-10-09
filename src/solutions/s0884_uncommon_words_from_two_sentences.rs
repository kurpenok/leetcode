use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut words_counter: HashMap<String, usize> = HashMap::new();

    for word in format!("{} {}", s1, s2).split_whitespace() {
        words_counter
            .entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut uncommon_words = Vec::new();
    for (word, count) in words_counter {
        if count == 1 {
            uncommon_words.push(word);
        }
    }
    uncommon_words.sort();

    uncommon_words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string()
            ),
            ["sour", "sweet"]
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            ["banana"]
        )
    }
}
