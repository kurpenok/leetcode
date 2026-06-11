#[cfg(test)]
mod test_s0275 {
    use leetcode::s0275_h_index_ii::h_index;

    #[test]
    fn test_case_1() {
        assert_eq!(h_index(vec![0, 1, 3, 5, 6]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(h_index(vec![1, 2, 100]), 2);
    }
}
