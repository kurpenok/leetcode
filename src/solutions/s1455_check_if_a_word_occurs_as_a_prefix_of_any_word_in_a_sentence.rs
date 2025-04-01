pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    sentence
        .split_whitespace()
        .position(|word| word.starts_with(&search_word))
        .map_or(-1, |pos| pos as i32 + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            ),
            2
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            is_prefix_of_word("i am tired".to_string(), "you".to_string()),
            -1
        );
    }
}
