pub fn count_segments(s: String) -> i32 {
    s.split_whitespace().collect::<Vec<&str>>().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_segments("Hello, my name is John".to_string()), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_segments("Hello".to_string()), 1);
    }
}
