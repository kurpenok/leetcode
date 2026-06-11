#[cfg(test)]
mod test_s1886 {
    use leetcode::s1886_determine_whether_matrix_can_be_obtained_by_rotation::find_rotation;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_rotation(vec![vec![0, 1], vec![1, 0]], vec![vec![1, 0], vec![0, 1]]),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_rotation(vec![vec![0, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 1]]),
            false
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            find_rotation(
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
                vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
            ),
            true
        );
    }
}
