#[cfg(test)]
mod test_s2129 {
    use leetcode::s2129_capitalize_the_title::capitalize_title;

    #[test]
    fn test_case_1() {
        assert_eq!(
            capitalize_title("capiTalIze tHe titLe".to_string()),
            "Capitalize The Title"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            capitalize_title("First leTTeR of EACH Word".to_string()),
            "First Letter of Each Word"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            capitalize_title("i lOve leetcode".to_string()),
            "i Love Leetcode"
        );
    }
}
