use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map: HashMap<char, usize> = HashMap::new();
    let mut t_map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        if s_map.contains_key(&c) {
            s_map.entry(c).and_modify(|v| *v += 1);
        } else {
            s_map.insert(c, 1);
        }
    }

    for c in t.chars() {
        if t_map.contains_key(&c) {
            t_map.entry(c).and_modify(|v| *v += 1);
        } else {
            t_map.insert(c, 1);
        }
    }

    s_map == t_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn test_case_2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        assert_eq!(is_anagram(s, t), false);
    }
}
