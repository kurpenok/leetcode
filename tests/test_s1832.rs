#[cfg(test)]
mod test_s1832 {
    use leetcode::s1832_check_if_the_sentence_is_pangram::check_if_pangram;

    #[test]
    fn test_case_1() {
        assert_eq!(
            check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_if_pangram("leetcode".to_string()), false);
    }
}
