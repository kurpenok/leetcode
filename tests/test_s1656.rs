#[cfg(test)]
mod test_s1656 {
    use leetcode::s1656_design_an_ordered_stream::OrderedStream;

    #[test]
    fn test_case_1() {
        let mut ordered_stream = OrderedStream::new(5);

        assert_eq!(
            ordered_stream.insert(3, "ccccc".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(ordered_stream.insert(1, "aaaaa".to_string()), ["aaaaa"]);
        assert_eq!(
            ordered_stream.insert(2, "bbbbb".to_string()),
            ["bbbbb", "ccccc"]
        );
        assert_eq!(
            ordered_stream.insert(5, "eeeee".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(
            ordered_stream.insert(4, "ddddd".to_string()),
            ["ddddd", "eeeee"]
        );
    }
}
