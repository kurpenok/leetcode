pub fn divisor_game(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(divisor_game(2), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(divisor_game(3), false);
    }
}
