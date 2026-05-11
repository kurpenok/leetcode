#[cfg(test)]
mod test_s0223 {
    use leetcode::s0223_rectangle_area::compute_area;

    #[test]
    fn test_case_1() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
