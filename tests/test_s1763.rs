#[cfg(test)]
mod test_s1763 {
    use leetcode::s1763_longest_nice_substring::longest_nice_substring;

    #[test]
    fn test_case_1() {
        assert_eq!(longest_nice_substring("YazaAay".to_string()), "aAa");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(longest_nice_substring("Bb".to_string()), "Bb");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(longest_nice_substring("c".to_string()), "");
    }
}
