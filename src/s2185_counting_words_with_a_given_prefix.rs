pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().filter(|&word| word.starts_with(&pref)).count() as i32
}
