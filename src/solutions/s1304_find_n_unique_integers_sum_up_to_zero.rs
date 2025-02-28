pub fn sum_zero(n: i32) -> Vec<i32> {
    (1..=n / 2)
        .flat_map(|i| [i, -i])
        .chain(if n % 2 != 0 { Some(0) } else { None })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(sum_zero(5), [1, -1, 2, -2, 0]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sum_zero(3), [1, -1, 0]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(sum_zero(1), [0]);
    }
}
