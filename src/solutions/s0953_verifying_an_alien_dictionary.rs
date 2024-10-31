use std::collections::HashMap;
use std::iter::zip;

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let order_chars: Vec<char> = order.chars().collect();
    let mut abc: HashMap<char, usize> = HashMap::new();
    for i in 0..26 {
        abc.insert(order_chars[i], i);
    }

    for i in 1..words.len() {
        let mut is_equal = true;

        for (c_1, c_2) in zip(words[i - 1].chars(), words[i].chars()) {
            if abc[&c_1] == abc[&c_2] {
                continue;
            } else if abc[&c_1] < abc[&c_2] {
                is_equal = false;
                break;
            } else if abc[&c_1] > abc[&c_2] {
                return false;
            }
        }

        if is_equal && words[i - 1].len() > words[i].len() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_alien_sorted(
                vec!["hello".to_string(), "leetcode".to_string()],
                "hlabcdefgijkmnopqrstuvwxyz".to_string()
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_alien_sorted(
                vec!["word".to_string(), "world".to_string(), "row".to_string()],
                "worldabcefghijkmnpqstuvxyz".to_string()
            ),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            is_alien_sorted(
                vec!["apple".to_string(), "app".to_string()],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            false
        );
    }
}
