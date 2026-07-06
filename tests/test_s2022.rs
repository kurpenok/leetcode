#[cfg(test)]
mod test_s2022 {
    use leetcode::s2022_convert_1d_array_into_2d_array::construct2_d_array;

    #[test]
    fn test_case_1() {
        assert_eq!(construct2_d_array(vec![1, 2, 3, 4], 2, 2), [[1, 2], [3, 4]]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(construct2_d_array(vec![1, 2, 3], 1, 3), [[1, 2, 3]]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            construct2_d_array(vec![1, 2], 1, 1),
            vec![] as Vec<Vec<i32>>
        );
    }
}
