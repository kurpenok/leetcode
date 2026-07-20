#[cfg(test)]
mod test_s2138 {
    use leetcode::s2138_divide_a_string_into_groups_of_size_k::divide_string;

    #[test]
    fn test_case_1() {
        assert_eq!(
            divide_string("abcdefghi".to_string(), 3, 'x'),
            ["abc", "def", "ghi"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            divide_string("abcdefghij".to_string(), 3, 'x'),
            ["abc", "def", "ghi", "jxx"]
        );
    }
}
