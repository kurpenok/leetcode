pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    patterns
        .iter()
        .filter(|&pattern| word.contains(pattern))
        .count() as i32
}
