#[cfg(test)]
mod test_s1736 {
    use leetcode::s1736_latest_time_by_replacing_hidden_digits::maximum_time;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum_time("2?:?0".to_string()), "23:50");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum_time("0?:3?".to_string()), "09:39");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(maximum_time("1?:22".to_string()), "19:22");
    }
}
