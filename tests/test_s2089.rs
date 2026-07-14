#[cfg(test)]
mod test_s2089 {
    use leetcode::s2089_find_target_indices_after_sorting_array::target_indices;

    #[test]
    fn test_case_1() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), [1, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 3), [3]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 5), [4]);
    }
}
