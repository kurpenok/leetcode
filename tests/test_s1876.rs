#[cfg(test)]
mod test_s1876 {
    use leetcode::s1876_substrings_of_size_three_with_distinct_characters::count_good_substrings;

    #[test]
    fn test_case_1() {
        assert_eq!(count_good_substrings("xyzzaz".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_good_substrings("aababcabc".to_string()), 4);
    }
}
