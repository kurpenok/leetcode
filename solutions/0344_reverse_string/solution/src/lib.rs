pub fn reverse_string(s: &mut Vec<char>) {
    let mut left: usize = 0;
    let mut right: usize = s.len() - 1;

    while left < right {
        let temp = s[left];

        s[left] = s[right];
        s[right] = temp;

        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut s: Vec<char> = "hello".to_string().chars().collect();
        reverse_string(&mut s);

        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_case_2() {
        let mut s: Vec<char> = "Hannah".to_string().chars().collect();
        reverse_string(&mut s);

        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
