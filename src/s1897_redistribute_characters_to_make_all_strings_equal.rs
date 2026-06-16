use std::collections::HashMap;

pub fn make_equal(words: Vec<String>) -> bool {
    words
        .iter()
        .flat_map(|word| word.chars())
        .fold(HashMap::new(), |mut chars_counter, c| {
            chars_counter
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            chars_counter
        })
        .values()
        .all(|&count| count % words.len() == 0)
}
