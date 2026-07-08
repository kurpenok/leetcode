#[cfg(test)]
mod test_s2062 {
    use leetcode::s2062_count_vowel_substrings_of_a_string::count_vowel_substrings;

    #[test]
    fn test_case_1() {
        assert_eq!(count_vowel_substrings("aeiouu".to_string()), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_vowel_substrings("unicornarihan".to_string()), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(count_vowel_substrings("cuaieuouac".to_string()), 7);
    }
}
