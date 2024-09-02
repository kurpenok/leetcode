pub fn valid_palindrome(s: String) -> bool {
    let symbols: Vec<char> = s.chars().collect();

    for i in 0..s.len() / 2 {
        let left = i;
        let right = s.len() - i - 1;

        if symbols[left] != symbols[right] {
            let mut new_s_1 = s.clone();
            let mut new_s_2 = s.clone();

            new_s_1.remove(left);
            new_s_2.remove(right);

            if new_s_1 == new_s_1.chars().rev().collect::<String>()
                || new_s_2 == new_s_2.chars().rev().collect::<String>()
            {
                return true;
            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(valid_palindrome("aba".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(valid_palindrome("abca".to_string()), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(valid_palindrome("abc".to_string()), false);
    }
}
