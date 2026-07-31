#[cfg(test)]
mod test_s2194 {
    use leetcode::s2194_cells_in_a_range_on_an_excel_sheet::cells_in_range;

    #[test]
    fn test_case_1() {
        assert_eq!(
            cells_in_range("K1:L2".to_string()),
            ["K1", "K2", "L1", "L2"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            cells_in_range("A1:F1".to_string()),
            ["A1", "B1", "C1", "D1", "E1", "F1"]
        );
    }
}
