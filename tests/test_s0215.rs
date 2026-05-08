#[cfg(test)]
mod test_s0215 {
    use leetcode::s0215_kth_largest_element_in_an_array::find_kth_largest;

    #[test]
    fn test_case_1() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
