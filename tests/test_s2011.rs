#[cfg(test)]
mod test_s2011 {
    use leetcode::s2011_final_value_of_variable_after_performing_operations::final_value_after_operations;

    #[test]
    fn test_case_1() {
        assert_eq!(
            final_value_after_operations(vec![
                "--X".to_string(),
                "X++".to_string(),
                "X++".to_string()
            ]),
            1
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            final_value_after_operations(vec![
                "++X".to_string(),
                "++X".to_string(),
                "X++".to_string()
            ]),
            3
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            final_value_after_operations(vec![
                "X++".to_string(),
                "++X".to_string(),
                "--X".to_string(),
                "X--".to_string()
            ]),
            0
        );
    }
}
