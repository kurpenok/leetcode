pub fn next_greater_letter(letters: Vec<char>, target: char) -> char {
    for letter in &letters {
        if *letter > target {
            return *letter;
        }
    }

    letters[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(next_greater_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    }

    #[test]
    fn test_case_2() {
        assert_eq!(next_greater_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    }

    #[test]
    fn test_case_3() {
        assert_eq!(next_greater_letter(vec!['x', 'x', 'y', 'y'], 'z'), 'x');
    }
}
