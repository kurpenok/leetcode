fn is_palindrome(s: &Vec<char>, start: usize, end: usize) -> bool {
    let mut start = start;
    let mut end = end;

    while start < end {
        if s[start] != s[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
}

fn helper(
    s_chars: &Vec<char>,
    cans: &mut Vec<String>,
    result: &mut Vec<Vec<String>>,
    index: usize,
) {
    if index == s_chars.len() {
        result.push(cans.clone());
    }

    for i in index..s_chars.len() {
        if !is_palindrome(s_chars, index, i) {
            continue;
        }

        let can: String = s_chars[index..i + 1].into_iter().collect();
        cans.push(can);
        helper(s_chars, cans, result, i + 1);
        cans.pop();
    }
}

pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut res = vec![];

    helper(&s.chars().collect(), &mut vec![], &mut res, 0);

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            partition("aab".to_string()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(partition("a".to_string()), vec![vec!["a"]]);
    }
}
