pub fn modify_string(s: String) -> String {
    let s_bytes = s.as_bytes();
    let mut prev: u8 = 0;

    let result = s_bytes
        .iter()
        .enumerate()
        .map(|(index, &b)| match b {
            b'?' => {
                let mut new_b = (prev + 1) % 26;
                if (index + 1) < s_bytes.len() && (new_b + b'a') == s_bytes[index + 1] {
                    new_b = (s_bytes[index + 1] - b'a' + 1) % 26;
                }
                prev = new_b;
                new_b + b'a'
            }
            _ => {
                prev = b;
                b
            }
        })
        .collect();

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(modify_string("?zs".to_string()), "bzs");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(modify_string("ubv?w".to_string()), "ubvpw");
    }
}
