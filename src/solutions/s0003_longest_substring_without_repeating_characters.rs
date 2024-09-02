use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let symbols: Vec<char> = s.chars().collect();
    let mut slice: HashMap<char, usize> = HashMap::new();

    let mut max_length: usize = 0;
    let mut i: usize = 0;

    while i < symbols.len() {
        if !slice.contains_key(&symbols[i]) {
            slice.insert(symbols[i], i);
            i += 1;
        } else {
            i = slice[&symbols[i]] + 1;
            if max_length < slice.len() {
                max_length = slice.len();
            }
            slice.clear();
        }
    }

    std::cmp::max(max_length, slice.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
