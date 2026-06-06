#[cfg(test)]
mod test_s0274 {
    use leetcode::s0274_h_index::h_index;

    #[test]
    fn test_case_1() {
        assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(h_index(vec![1, 3, 1]), 1);
    }
}
