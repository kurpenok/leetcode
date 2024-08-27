pub fn to_lower_case(s: String) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(to_lower_case("Hello".to_string()), "hello");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(to_lower_case("here".to_string()), "here");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(to_lower_case("LOVELY".to_string()), "lovely");
    }
}
