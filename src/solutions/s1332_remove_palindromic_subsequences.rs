pub fn remove_palindrome_sub(s: String) -> i32 {
    let s = s.as_bytes();

    if s.is_empty() {
        0
    } else if (0..s.len() / 2).all(|i| s[i] == s[s.len() - 1 - i]) {
        1
    } else {
        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(remove_palindrome_sub("ababa".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(remove_palindrome_sub("abb".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(remove_palindrome_sub("baabb".to_string()), 2);
    }
}
