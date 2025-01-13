pub fn balanced_string_split(s: String) -> i32 {
    s.chars()
        .fold((0, 0), |(count, balance), c| match c {
            'L' => (count + (balance == 0) as i32, balance + 1),
            'R' => (count + (balance == 0) as i32, balance - 1),
            _ => unreachable!(),
        })
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(balanced_string_split("RLRRLLRLRL".to_string()), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(balanced_string_split("RLRRRLLRLL".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(balanced_string_split("LLLLRRRR".to_string()), 1);
    }
}
