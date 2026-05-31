#[cfg(test)]
mod test_s1812 {
    use leetcode::s1812_determine_color_of_a_chessboard_square::square_is_white;

    #[test]
    fn test_case_1() {
        assert_eq!(square_is_white("a1".to_string()), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(square_is_white("h3".to_string()), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(square_is_white("c7".to_string()), false);
    }
}
