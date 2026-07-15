#[cfg(test)]
mod test_s2103 {
    use leetcode::s2103_rings_and_rods::count_points;

    #[test]
    fn test_case_1() {
        assert_eq!(count_points("B0B6G0R6R0R6G9".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_points("B0R0G0R9R0B0G0".to_string()), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(count_points("G4".to_string()), 0);
    }
}
