use std::collections::HashMap;

fn can_be_built(word: &String, chars_map: &HashMap<char, usize>) -> bool {
    let mut word_map: HashMap<char, usize> = HashMap::new();
    for c in word.chars() {
        word_map
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (c, count) in &word_map {
        match chars_map.get(c) {
            Some(chars_map_c) => {
                if chars_map_c < count {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut sum_of_lengths: i32 = 0;

    let mut chars_map: HashMap<char, usize> = HashMap::new();
    for c in chars.chars() {
        chars_map
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for word in &words {
        if can_be_built(word, &chars_map) {
            sum_of_lengths += word.len() as i32;
        }
    }

    sum_of_lengths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string(),
                ],
                "atach".to_string(),
            ),
            6,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string(),
                ],
                "welldonehoneyr".to_string(),
            ),
            10,
        );
    }
}
