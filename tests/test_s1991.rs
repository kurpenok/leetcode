#[cfg(test)]
mod test_s1991 {
    use leetcode::s1991_find_the_middle_index_in_array::find_middle_index;

    #[test]
    fn test_case_1() {
        assert_eq!(find_middle_index(vec![2, 3, -1, 8, 4]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_middle_index(vec![1, -1, 4]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_middle_index(vec![2, 5]), -1);
    }
}
