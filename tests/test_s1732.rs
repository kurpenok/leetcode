#[cfg(test)]
mod test_s1732 {
    use leetcode::s1732_find_the_highest_altitude::largest_altitude;

    #[test]
    fn test_case_1() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
