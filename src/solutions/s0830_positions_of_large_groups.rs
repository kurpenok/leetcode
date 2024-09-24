pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut start: usize = 0;
    let mut end: usize = 0;

    let s_symbols = s.chars().collect::<Vec<char>>();
    for i in 1..s_symbols.len() {
        if s_symbols[i - 1] == s_symbols[i] {
            end += 1;
        } else if s_symbols[i - 1] != s_symbols[i] && end - start + 1 > 2 {
            result.push(vec![start as i32, end as i32]);
            start = i;
            end = i;
        } else {
            start = i;
            end = i;
        }
    }

    if end - start + 1 > 2 {
        result.push(vec![start as i32, end as i32]);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            large_group_positions("abbxxxxzzy".to_string()),
            vec![vec![3, 6]]
        );
    }

    #[test]
    fn test_case_2() {
        let result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(large_group_positions("abc".to_string()), result);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            large_group_positions("abcdddeeeeaabbbcd".to_string()),
            vec![vec![3, 5], vec![6, 9], vec![12, 14]]
        );
    }
}
