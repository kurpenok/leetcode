#[cfg(test)]
mod test_s1688 {
    use leetcode::s1688_count_of_matches_in_tournament::number_of_matches;

    #[test]
    fn test_case_1() {
        assert_eq!(number_of_matches(7), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(number_of_matches(14), 13);
    }
}
