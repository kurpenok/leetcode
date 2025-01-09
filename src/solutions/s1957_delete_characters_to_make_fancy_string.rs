pub fn make_fancy_string(s: String) -> String {
    if s.len() < 3 {
        s
    } else {
        let s_chars: Vec<char> = s.chars().collect();

        let mut fancy_string = String::new();
        fancy_string.push(s_chars[0]);
        fancy_string.push(s_chars[1]);

        for i in 2..s.len() {
            if s_chars[i - 2] == s_chars[i - 1]
                && s_chars[i - 2] == s_chars[i]
                && s_chars[i - 1] == s_chars[i]
            {
                continue;
            }
            fancy_string.push(s_chars[i]);
        }

        fancy_string
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(make_fancy_string("leeetcode".to_string()), "leetcode");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(make_fancy_string("aaabaaaa".to_string()), "aabaa");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(make_fancy_string("aab".to_string()), "aab");
    }
}
