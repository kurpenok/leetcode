pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix: String = String::new();

    let zero_symbols: Vec<char> = strs[0].chars().collect();
    for i in 0..strs[0].len() {
        for str in &strs {
            let symbols: Vec<char> = str.chars().collect();
            if i == str.len() || zero_symbols[i] != symbols[i] {
                return prefix;
            }
        }
        prefix.push(zero_symbols[i]);
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
