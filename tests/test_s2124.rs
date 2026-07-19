#[cfg(test)]
mod test_s2124 {
    use leetcode::s2124_check_if_all_a_s_appears_before_all_b_s::check_string;

    #[test]
    fn test_case_1() {
        assert_eq!(check_string("aaabbb".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_string("abab".to_string()), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(check_string("bbb".to_string()), true);
    }
}
