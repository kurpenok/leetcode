use std::collections::HashMap;

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let words = paragraph
        .to_lowercase()
        .split(&[' ', '!', '?', '\'', ',', ';', '.'])
        .filter(|&r| r != "")
        .map(|r| r.to_string())
        .collect::<Vec<String>>();

    let mut max_count = 0;
    let mut max_count_word = String::new();
    let mut word_counter: HashMap<String, usize> = HashMap::new();

    words.iter().for_each(|word| {
        word_counter
            .entry(word.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });

    for (word, count) in &word_counter {
        if banned
            .iter()
            .position(|banned_word| *banned_word == *word)
            .is_none()
            && max_count < *count
        {
            max_count = *count;
            max_count_word = word.clone();
        }
    }

    max_count_word
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(most_common_word("a.".to_string(), vec![]), "a");
    }
}
