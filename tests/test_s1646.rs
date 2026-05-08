#[cfg(test)]
mod test_s1646 {
    use leetcode::s1646_get_maximum_in_generated_array::get_maximum_generated;

    #[test]
    fn test_case_1() {
        assert_eq!(get_maximum_generated(7), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(get_maximum_generated(2), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(get_maximum_generated(3), 2);
    }
}
