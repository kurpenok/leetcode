use std::collections::HashSet;

pub fn count_vowel_substrings(word: String) -> i32 {
    let chars = word.chars().collect::<Vec<char>>();

    (0..chars.len())
        .flat_map(|start| {
            chars[start..]
                .iter()
                .take_while(|&&c| "aeiou".contains(c))
                .scan(HashSet::new(), |seen, &c| {
                    seen.insert(c);
                    Some(seen.len())
                })
        })
        .filter(|&unique_vowels_count| unique_vowels_count == 5)
        .count() as i32
}
