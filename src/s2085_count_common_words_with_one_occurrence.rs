use std::collections::HashMap;

pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let unique_words_1 = words1
        .iter()
        .fold(HashMap::new(), |mut unique_words, word| {
            *unique_words.entry(word).or_insert(0) += 1;
            unique_words
        });
    let unique_words_2 = words2
        .iter()
        .fold(HashMap::new(), |mut unique_words, word| {
            *unique_words.entry(word).or_insert(0) += 1;
            unique_words
        });

    unique_words_1
        .iter()
        .filter(|&(unique_word, &count)| {
            count == 1
                && unique_words_2.contains_key(unique_word)
                && unique_words_2[unique_word] == 1
        })
        .count() as i32
}
