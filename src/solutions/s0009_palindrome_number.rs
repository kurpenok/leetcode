pub fn is_palindrome(x: i32) -> bool {
    x.to_string().chars().collect::<Vec<char>>()
        == x.to_string().chars().rev().collect::<Vec<char>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_palindrome(10), false);
    }
}
