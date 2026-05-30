use std::collections::HashSet;

pub fn num_different_integers(word: String) -> i32 {
    word.chars()
        .map(|c| if c.is_ascii_digit() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|number| number.trim_start_matches('0'))
        .collect::<HashSet<&str>>()
        .len() as i32
}
