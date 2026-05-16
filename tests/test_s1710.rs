#[cfg(test)]
mod test_s1710 {
    use leetcode::s1710_maximum_units_on_a_truck::maximum_units;

    #[test]
    fn test_case_1() {
        assert_eq!(
            maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
            91
        );
    }
}
