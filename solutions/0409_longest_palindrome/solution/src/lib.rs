use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let symbols: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut map, c| {
        let _ = *map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    let evens: Vec<i32> = symbols
        .clone()
        .values()
        .filter(|&value| value % 2 == 0)
        .map(|&value| value as i32)
        .collect();

    let odds: Vec<i32> = symbols
        .clone()
        .values()
        .filter(|&value| value % 2 != 0)
        .map(|&value| value as i32 - 1)
        .collect();

    if odds.len() > 0 {
        return evens.into_iter().sum::<i32>() + odds.into_iter().sum::<i32>() + 1;
    }

    evens.into_iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(longest_palindrome("bananas".to_string()), 5);
    }
}
