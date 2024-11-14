pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut min_freq = vec![i32::MAX; 26];

    for word in words {
        let mut freq = vec![0; 26];

        for ch in word.chars() {
            if let Some(index) = (ch as usize).checked_sub('a' as usize) {
                if index < 26 {
                    freq[index] += 1;
                }
            }
        }

        for i in 0..26 {
            min_freq[i] = min_freq[i].min(freq[i]);
        }
    }

    let mut result = Vec::new();

    for i in 0..26 {
        for _ in 0..min_freq[i] {
            result.push(((i as u8 + 'a' as u8) as char).to_string());
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            ["e", "l", "l"],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ]),
            ["c", "o"],
        );
    }
}
