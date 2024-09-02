use std::collections::HashMap;

pub fn find_the_difference(s: String, t: String) -> char {
    let mut t_chars: HashMap<char, usize> = HashMap::new();
    for c in t.chars() {
        t_chars
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for c in s.chars() {
        t_chars.entry(c).and_modify(|count| *count -= 1);
    }

    for (key, value) in t_chars.iter() {
        if *value == 1 {
            return *key;
        }
    }

    'a'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_the_difference("".to_string(), "y".to_string()), 'y');
    }
}
