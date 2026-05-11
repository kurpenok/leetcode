#[cfg(test)]
mod test_s1678 {
    use leetcode::s1678_goal_parser_interpretation::interpret;

    #[test]
    fn test_case_1() {
        assert_eq!(interpret("G()(al)".to_string()), "Goal");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(interpret("G()()()()(al)".to_string()), "Gooooal");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(interpret("(al)G(al)()()G".to_string()), "alGalooG");
    }
}
