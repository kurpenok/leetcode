pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut merged_word = String::with_capacity(word1.len() + word2.len());
    let word1_chars: Vec<char> = word1.chars().collect();
    let word2_chars: Vec<char> = word2.chars().collect();

    let mut i = 0;
    let mut j = 0;

    while i < word1.len() && j < word2.len() {
        merged_word.push(word1_chars[i]);
        merged_word.push(word2_chars[j]);
        i += 1;
        j += 1;
    }

    while i < word1.len() {
        merged_word.push(word1_chars[i]);
        i += 1;
    }

    while j < word2.len() {
        merged_word.push(word2_chars[j]);
        j += 1;
    }

    merged_word
}
