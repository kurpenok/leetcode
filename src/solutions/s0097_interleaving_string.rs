pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let s1_len = s1.len();
    let s2_len = s2.len();
    let s3_len = s3.len();

    if s3_len != s1_len + s2_len {
        return false;
    }

    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let s3: Vec<char> = s3.chars().collect();

    let mut dp = vec![false; s2_len + 1];
    dp[0] = true;

    for j in 1..=s2_len {
        dp[j] = dp[j - 1] && s2[j - 1] == s3[j - 1];
    }

    for i in 1..=s1_len {
        let mut curr_dp = vec![false; s2_len + 1];
        curr_dp[0] = dp[0] && s1[i - 1] == s3[i - 1];

        for j in 1..=s2_len {
            let k = i + j;
            let current_char = s3[k - 1];

            let from_s1 = dp[j] && s1[i - 1] == current_char;
            let from_s2 = curr_dp[j - 1] && s2[j - 1] == current_char;

            curr_dp[j] = from_s1 || from_s2;
        }

        dp = curr_dp;
    }

    dp[s2_len]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
    }

    #[test]
    fn test_case_2() {
        assert!(!is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
    }

    #[test]
    fn test_case_3() {
        assert!(is_interleave(
            "".to_string(),
            "".to_string(),
            "".to_string()
        ));
    }
}
