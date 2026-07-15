pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count())
        .max()
        .unwrap() as i32
}
