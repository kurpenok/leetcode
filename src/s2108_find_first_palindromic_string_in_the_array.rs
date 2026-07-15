pub fn first_palindrome(words: Vec<String>) -> String {
    words
        .into_iter()
        .find(|word| word.chars().eq(word.chars().rev()))
        .unwrap_or_default()
}
