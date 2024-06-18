use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut chars: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        chars.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }

    for (i, c) in s.chars().into_iter().enumerate() {
        if chars[&c] == 1 {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(first_uniq_char("aabb".to_string()), -1);
    }
}
