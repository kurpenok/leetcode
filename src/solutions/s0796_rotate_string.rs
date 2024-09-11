pub fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len() && goal.repeat(2).contains(s.as_str())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(rotate_string("abcde".to_string(), "cdeab".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(rotate_string("abcde".to_string(), "abced".to_string()), false);
    }
}
