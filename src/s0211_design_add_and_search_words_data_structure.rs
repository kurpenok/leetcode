pub struct WordDictionary {
    words: Vec<String>,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary { words: Vec::new() }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn search(&self, word: String) -> bool {
        self.words.iter().any(|w| {
            w.len() == word.len()
                && w.chars()
                    .zip(word.chars())
                    .all(|(w_char, word_char)| word_char == '.' || w_char == word_char)
        })
    }
}
