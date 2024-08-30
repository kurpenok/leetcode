use std::collections::HashMap;

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut symbols: HashMap<char, usize> = HashMap::new();

    license_plate
        .to_lowercase()
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .for_each(|symbol| {
            if symbol.is_ascii_lowercase() {
                symbols
                    .entry(*symbol)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        });

    let mut result = "".to_string();

    for word in words {
        let mut temp_symbols = symbols.clone();

        for symbol in word.chars() {
            if !temp_symbols.contains_key(&symbol) || temp_symbols[&symbol] as i32 - 1 < 0 {
                continue;
            }
            temp_symbols.entry(symbol).and_modify(|count| *count -= 1);
        }

        if temp_symbols.values().all(|count| *count == 0)
            && (result == "".to_string() || result.len() > word.len())
        {
            result = word.clone();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            shortest_completing_word(
                "1s3 PSt".to_string(),
                vec![
                    "step".to_string(),
                    "steps".to_string(),
                    "stripe".to_string(),
                    "stepple".to_string()
                ]
            ),
            "steps"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            shortest_completing_word(
                "1s3 456".to_string(),
                vec![
                    "looks".to_string(),
                    "pest".to_string(),
                    "stew".to_string(),
                    "show".to_string()
                ]
            ),
            "pest"
        );
    }
}
