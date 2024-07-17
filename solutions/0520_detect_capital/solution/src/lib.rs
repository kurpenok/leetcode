fn to_first_capitalize(word: String) -> String {
    let mut v: Vec<char> = word.to_lowercase().chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}

pub fn detect_capital_use(word: String) -> bool {
    if word == word.to_uppercase()
        || word == word.to_lowercase()
        || word == to_first_capitalize(word.clone())
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(detect_capital_use("USA".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(detect_capital_use("FlaG".to_string()), false);
    }
}
