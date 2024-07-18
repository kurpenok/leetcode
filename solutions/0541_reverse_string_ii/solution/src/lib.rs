pub fn reverse_str(s: String, k: i32) -> String {
    let k = k as usize;
    let mut s: Vec<char> = s.chars().collect();

    for start in (0..s.len()).step_by(2 * k) {
        let mut i = start;
        let mut j = (start + k - 1).min(s.len() - 1);

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    s.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(reverse_str("abcdefg".to_string(), 2), "bacdfeg");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reverse_str("abcd".to_string(), 2), "bacd");
    }
}
