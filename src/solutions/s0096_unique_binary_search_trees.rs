fn catalan(n: f64) -> f64 {
    if n <= 1.0 {
        1.0
    } else {
        2.0 * (2.0 * n - 1.0) / (n + 1.0) * catalan(n - 1.0)
    }
}

pub fn num_trees(n: i32) -> i32 {
    catalan(n as f64) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(num_trees(3), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(num_trees(1), 1);
    }
}
