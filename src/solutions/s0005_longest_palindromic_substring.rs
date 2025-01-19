fn is_palindrome(s_chars: &Vec<char>, i: usize, j: usize) -> bool {
    let mut left = i;
    let mut right = j - 1;

    while left < right {
        if s_chars[left] != s_chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

pub fn longest_palindrome(s: String) -> String {
    let s_chars: Vec<char> = s.chars().collect();

    for length in (1..s.len() + 1).rev() {
        for start in 0..s.len() - length + 1 {
            if is_palindrome(&s_chars, start, start + length) {
                return s[start..start + length].to_string();
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    }
}
