pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    text.split_whitespace()
        .filter(|&word| !word.chars().any(|c| broken_letters.contains(c)))
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            can_be_typed_words("leet code".to_string(), "e".to_string()),
            0,
        );
    }
}
