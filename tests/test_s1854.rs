#[cfg(test)]
mod test_s1854 {
    use leetcode::s1854_maximum_population_year::maximum_population;

    #[test]
    fn test_case_1() {
        assert_eq!(
            maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
            1993
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]),
            1960
        );
    }
}
