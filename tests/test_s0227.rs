#[cfg(test)]
mod test_s0227 {
    use leetcode::s0227_basic_calculator_ii::calculate;

    #[test]
    fn test_case_1() {
        assert_eq!(calculate("3+2*2".to_string()), 7);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(calculate("3/2 ".to_string()), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(calculate(" 3+5 / 2 ".to_string()), 5);
    }
}
