pub fn thousand_separator(n: i32) -> String {
    n.to_string()
        .chars()
        .collect::<Vec<char>>()
        .rchunks(3)
        .rev()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(".")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(thousand_separator(987), "987");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(thousand_separator(1234), "1.234");
    }
}
