use std::collections::HashSet;

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let keyrow_1: HashSet<char> = HashSet::from_iter("qwertyuiop".chars().collect::<Vec<char>>());
    let keyrow_2: HashSet<char> = HashSet::from_iter("asdfghjkl".chars().collect::<Vec<char>>());
    let keyrow_3: HashSet<char> = HashSet::from_iter("zxcvbnm".chars().collect::<Vec<char>>());

    for word in words {
        let chars_in_word: HashSet<char> =
            HashSet::from_iter(word.to_lowercase().chars().collect::<Vec<char>>());
        if chars_in_word.is_subset(&keyrow_1)
            || chars_in_word.is_subset(&keyrow_2)
            || chars_in_word.is_subset(&keyrow_3)
        {
            result.push(word);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ]),
            vec!["Alaska", "Dad"]
        );
    }

    #[test]
    fn test_case_2() {
        let result: Vec<String> = Vec::new();
        assert_eq!(find_words(vec!["omk".to_string()]), result);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            find_words(vec!["adsdf".to_string(), "sfd".to_string()]),
            vec!["adsdf", "sfd"],
        );
    }
}
