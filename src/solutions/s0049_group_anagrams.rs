use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();

        let key: String = chars.into_iter().collect();

        groups.entry(key).or_insert(Vec::new()).push(s.clone());
    }

    let mut groups: Vec<Vec<String>> = groups.into_values().collect();
    groups.sort();
    groups
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(group_anagrams(vec!["".to_string()]), [[""]]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(group_anagrams(vec!["a".to_string()]), [["a"]]);
    }
}
