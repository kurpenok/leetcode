#[cfg(test)]
mod test_s1925 {
    use leetcode::s1925_count_square_sum_triple::count_triples;

    #[test]
    fn test_case_1() {
        assert_eq!(count_triples(5), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_triples(10), 4);
    }
}
