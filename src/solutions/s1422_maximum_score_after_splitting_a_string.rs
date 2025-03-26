pub fn max_score(s: String) -> i32 {
    let mut final_score: usize = 0;

    for i in 1..s.len() {
        final_score = std::cmp::max(
            final_score,
            s.as_bytes()[..i].iter().filter(|&c| *c == b'0').count()
                + s.as_bytes()[i..].iter().filter(|&c| *c == b'1').count(),
        );
    }

    final_score as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_score("011101".to_string()), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_score("00111".to_string()), 5);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_score("1111".to_string()), 3);
    }
}
