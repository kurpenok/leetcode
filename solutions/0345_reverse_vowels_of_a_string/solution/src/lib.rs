pub fn reverse_vowels(s: String) -> String {
    let mut vowels: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == 'a'
            || c == 'e'
            || c == 'i'
            || c == 'o'
            || c == 'u'
            || c == 'A'
            || c == 'E'
            || c == 'I'
            || c == 'O'
            || c == 'U'
        {
            vowels.push(c);
        }
    }

    let mut vowel_index: usize = vowels.len();
    let mut new_string: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == 'a'
            || c == 'e'
            || c == 'i'
            || c == 'o'
            || c == 'u'
            || c == 'A'
            || c == 'E'
            || c == 'I'
            || c == 'O'
            || c == 'U'
        {
            new_string.push(vowels[vowel_index - 1]);
            vowel_index -= 1;
        } else {
            new_string.push(c);
        }
    }

    new_string.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle".to_string());
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(reverse_vowels("aA".to_string()), "Aa".to_string());
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            reverse_vowels("Yo! Bottoms up, U.S. Motto, boy!".to_string()),
            "Yo! Bottoms Up, u.S. Motto, boy!".to_string()
        );
    }
}
