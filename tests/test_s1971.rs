#[cfg(test)]
mod test_s1971 {
    use leetcode::s1971_find_if_path_exists_in_graph::valid_path;

    #[test]
    fn test_case_1() {
        assert_eq!(
            valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            valid_path(
                6,
                vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
                0,
                5
            ),
            false
        );
    }
}
