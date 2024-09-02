pub fn is_subsequence(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < s_chars.len() {
        while j < t_chars.len() {
            if s_chars[i] == t_chars[j] {
                i += 1;
                j += 1;
                break;
            }
            j += 1;
        }

        if i < s_chars.len() && j == t_chars.len() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            is_subsequence("acb".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()),
            false
        );
    }
}
