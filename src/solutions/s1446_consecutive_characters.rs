pub fn max_power(s: String) -> i32 {
    let mut power: i32 = 1;
    let mut result: i32 = power;

    let s_chars: Vec<char> = s.chars().collect();
    for i in 1..s.len() {
        if s_chars[i - 1] == s_chars[i] {
            power += 1;
        } else {
            result = std::cmp::max(power, result);
            power = 1;
        }
    }

    std::cmp::max(power, result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_power("leetcode".to_string()), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_power("abbcccddddeeeeedcba".to_string()), 5);
    }
}
