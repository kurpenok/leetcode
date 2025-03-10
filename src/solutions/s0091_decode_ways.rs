pub fn num_decodings(s: String) -> i32 {
    let n = s.len();

    let mut dp = vec![0; n + 1];
    dp[n] = 1;

    let s = s.as_bytes();
    if s[n - 1] != b'0' {
        dp[n - 1] = 1;
    }

    for i in (0..n - 1).rev() {
        if s[i] != b'0' {
            dp[i] += dp[i + 1];
        }
        if s[i] == b'1' || (s[i] == b'2' && s[i + 1] < b'7') {
            dp[i] += dp[i + 2];
        }
    }

    dp[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(num_decodings("06".to_string()), 0);
    }
}
