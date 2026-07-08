#[cfg(test)]
mod test_s2047 {
    use leetcode::s2047_number_of_valid_words_in_a_sentence::count_valid_words;

    #[test]
    fn test_case_1() {
        assert_eq!(count_valid_words("cat and  dog".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_valid_words("!this  1-s b8d!".to_string()), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            count_valid_words("alice and  bob are playing stone-game10".to_string()),
            5
        );
    }
}
