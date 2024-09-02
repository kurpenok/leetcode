pub fn arrange_coins(n: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Triangular_number

    let d = 8 * (n as i64) + 1;
    let d = (d as f64).sqrt() as i32;

    (d - 1) >> 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(arrange_coins(5), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(arrange_coins(8), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(arrange_coins(15), 5);
    }
}
