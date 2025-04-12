fn is_operator(s: &str) -> bool {
    s == "+" || s == "-" || s == "*" || s == "/"
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        if is_operator(&token) {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => 0,
            };
            stack.push(result);
        } else {
            let num: i32 = token.parse().unwrap();
            stack.push(num);
        }
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
