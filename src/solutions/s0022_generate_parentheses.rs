fn generate(parentheses: &mut Vec<String>, s: &mut String, open: i32, close: i32, n: i32) {
    if open == n && close == n {
        parentheses.push(s.clone());
        return;
    }

    if open < n {
        s.push('(');
        generate(parentheses, s, open + 1, close, n);
        s.pop();
    }

    if close < open {
        s.push(')');
        generate(parentheses, s, open, close + 1, n);
        s.pop();
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut parentheses: Vec<String> = Vec::new();
    let mut s = String::new();
    generate(&mut parentheses, &mut s, 0, 0, n);
    parentheses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(generate_parenthesis(1), ["()"]);
    }
}
