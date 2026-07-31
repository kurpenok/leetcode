#[cfg(test)]
mod test_s2200 {
    use leetcode::s2200_find_all_k_distant_indices_in_an_array::find_k_distant_indices;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            [1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            [0, 1, 2, 3, 4]
        );
    }
}
