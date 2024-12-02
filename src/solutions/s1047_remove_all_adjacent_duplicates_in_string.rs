use std::collections::VecDeque;

pub fn remove_duplicates(s: String) -> String {
    let mut stack = VecDeque::new();

    for c in s.chars() {
        let last = stack.pop_back();
        match last {
            Some(last) => {
                if last != c {
                    stack.push_back(last);
                    stack.push_back(c);
                }
            }
            None => stack.push_back(c),
        }
    }

    stack.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(remove_duplicates("abbaca".to_string()), "ca");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(remove_duplicates("azxxzy".to_string()), "ay");
    }
}
