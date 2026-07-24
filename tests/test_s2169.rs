#[cfg(test)]
mod test_s2169 {
    use leetcode::s2169_count_operations_to_obtain_zero::count_operations;

    #[test]
    fn test_case_1() {
        assert_eq!(count_operations(2, 3), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_operations(10, 10), 1);
    }
}
