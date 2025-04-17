pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut shuffled_s: Vec<char> = vec![' '; s.len()];

    s.chars().zip(indices).for_each(|(c, i)| {
        shuffled_s[i as usize] = c;
    });

    shuffled_s.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(restore_string("abc".to_string(), vec![0, 1, 2]), "abc");
    }
}
