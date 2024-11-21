pub fn remove_outer_parentheses(s: String) -> String {
    let mut result = String::new();
    let mut balance = 0;

    for c in s.chars() {
        if c == '(' {
            if balance > 0 {
                result.push(c);
            }
            balance += 1;
        } else if c == ')' {
            balance -= 1;
            if balance > 0 {
                result.push(c);
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(remove_outer_parentheses("(()())(())".to_string()), "()()()");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(remove_outer_parentheses("()()".to_string()), "");
    }
}
