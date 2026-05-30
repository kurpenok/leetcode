#[cfg(test)]
mod test_s1805 {
    use leetcode::s1805_number_of_different_integers_in_a_string::num_different_integers;

    #[test]
    fn test_case_1() {
        assert_eq!(num_different_integers("a123bc34d8ef34".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(num_different_integers("leet1234code234".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(num_different_integers("a1b01c001".to_string()), 1);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(num_different_integers("167278959591294".to_string()), 1);
    }
}
