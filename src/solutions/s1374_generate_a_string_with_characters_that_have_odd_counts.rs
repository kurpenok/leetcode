pub fn generate_the_string(n: i32) -> String {
    if n % 2 == 0 {
        format!("{}{}", "a".repeat(n as usize - 1), "b")
    } else {
        format!("{}", "a".repeat(n as usize))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(generate_the_string(4), "aaab");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(generate_the_string(2), "ab");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(generate_the_string(7), "aaaaaaa");
    }
}
