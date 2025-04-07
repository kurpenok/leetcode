pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    let max_len = word_dict.iter().map(|word| word.len()).max().unwrap_or(0);
    let word_dict: std::collections::HashSet<String> = word_dict.into_iter().collect();

    for i in 1..=s.len() {
        for j in (std::cmp::max(i as i32 - max_len as i32 - 1, 0) as usize..i).rev() {
            if dp[j] && word_dict.contains(&s[j..i].to_string()) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[s.len()]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }
}
