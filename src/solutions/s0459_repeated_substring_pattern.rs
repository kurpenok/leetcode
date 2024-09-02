pub fn repeated_substring_pattern(s: String) -> bool {
    for i in 1..s.len() {
        let slice = &s[0..i].to_string();
        if s.len() % slice.len() != 0 {
            continue;
        }

        let new_s = slice.repeat(s.len() / slice.len());
        if s == new_s {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(repeated_substring_pattern("abab".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(repeated_substring_pattern("aba".to_string()), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(repeated_substring_pattern("abcabcabcabc".to_string()), true);
    }
}
