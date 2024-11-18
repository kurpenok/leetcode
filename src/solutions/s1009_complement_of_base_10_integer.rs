pub fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut n = n;
    let bits_count = ((n as f64).log2() + 1.0) as usize;

    for i in 0..bits_count {
        n = n ^ (1 << i);
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(bitwise_complement(5), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(bitwise_complement(7), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(bitwise_complement(10), 5);
    }
}
