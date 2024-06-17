use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine_symbols: HashMap<char, u32> = HashMap::new();
    for symbol in magazine.chars() {
        magazine_symbols
            .entry(symbol)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for symbol in ransom_note.chars() {
        if !magazine_symbols.contains_key(&symbol)
            || (magazine_symbols.contains_key(&symbol) && magazine_symbols[&symbol] < 1)
        {
            return false;
        }

        magazine_symbols
            .entry(symbol)
            .and_modify(|count| *count -= 1);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(can_construct("a".to_string(), "b".to_string()), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(can_construct("aa".to_string(), "aab".to_string()), true);
    }
}
