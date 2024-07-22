pub fn reverse_words(s: String) -> String {
    let mut result: Vec<String> = Vec::new();

    for word in s.split_whitespace() {
        result.push(word.chars().rev().collect::<String>())
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reverse_words("Mr Ding".to_string()), "rM gniD");
    }
}
