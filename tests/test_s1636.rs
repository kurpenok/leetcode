#[cfg(test)]
mod test_s1636 {
    use leetcode::s1636_sort_array_by_increasing_frequency::frequency_sort;

    #[test]
    fn test_case_1() {
        assert_eq!(frequency_sort(vec![1, 1, 2, 2, 2, 3]), [3, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(frequency_sort(vec![2, 3, 1, 3, 2]), [1, 3, 3, 2, 2]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            [5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
