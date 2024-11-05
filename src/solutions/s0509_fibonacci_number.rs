pub fn fib(n: i32) -> i32 {
    let sqrt_5 = 5_f64.sqrt();
    let phi = (sqrt_5 + 1.) / 2.;
    (phi.powi(n) / sqrt_5 + 0.5) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(fib(4), 3);
    }
}
