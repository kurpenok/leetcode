use std::collections::{HashMap, HashSet};

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut patterns: HashMap<char, &str> = HashMap::new();
    let mut unique_words: HashSet<String> = HashSet::new();

    let mut chars: Vec<char> = Vec::new();
    for c in pattern.chars() {
        chars.push(c);
    }

    let mut words: Vec<String> = Vec::new();
    for word in s.split_whitespace() {
        words.push(word.to_string());
    }

    if chars.len() != words.len() {
        return false;
    }

    for i in 0..chars.len() {
        if (patterns.contains_key(&chars[i]) && patterns[&chars[i]] != words[i])
            || (!patterns.contains_key(&chars[i]) && unique_words.contains(&words[i]))
        {
            return false;
        } else if !patterns.contains_key(&chars[i]) && !unique_words.contains(&words[i]) {
            patterns.insert(chars[i], &words[i]);
            unique_words.insert(words[i].clone());
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(pattern, s), true);
    }

    #[test]
    fn test_case_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }

    #[test]
    fn test_case_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }

    #[test]
    fn test_case_4() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }
}
