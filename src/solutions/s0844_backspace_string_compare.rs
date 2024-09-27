pub fn backspace_compare(s: String, t: String) -> bool {
    let mut new_s = String::new();
    let mut new_t = String::new();

    for c in s.chars() {
        if c == '#' && new_s.len() > 0 {
            new_s = new_s[..new_s.len() - 1].to_string();
            continue;
        } else if c == '#' {
            continue;
        }
        new_s.push(c);
    }

    for c in t.chars() {
        if c == '#' && new_t.len() > 0 {
            new_t = new_t[..new_t.len() - 1].to_string();
            continue;
        } else if c == '#' {
            continue;
        }
        new_t.push(c);
    }

    new_s == new_t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(backspace_compare("a#c".to_string(), "b".to_string()), false);
    }
}
