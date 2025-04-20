pub fn make_good(s: String) -> String {
    s.chars()
        .fold(String::new(), |mut result, c_2| match (result.pop(), c_2) {
            (None, c_2) => c_2.to_string(),
            (Some(c_1), c_2) if c_1 != c_2 && c_1.to_lowercase().eq(c_2.to_lowercase()) => result,
            (Some(c_1), c_2) => result + &c_1.to_string() + &c_2.to_string(),
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(make_good("leEeetcode".to_string()), "leetcode");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(make_good("abBAcC".to_string()), "");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(make_good("s".to_string()), "s");
    }
}
