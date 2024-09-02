pub fn check_perfect_number(num: i32) -> bool {
    vec![6, 28, 496, 8128, 33550336].contains(&num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(check_perfect_number(28), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_perfect_number(7), false);
    }
}
