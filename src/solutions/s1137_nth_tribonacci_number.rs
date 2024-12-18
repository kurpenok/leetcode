pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 1;

    for _ in 3..=n {
        let next = a + b + c;
        a = b;
        b = c;
        c = next;
    }

    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(tribonacci(4), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(tribonacci(25), 1389537);
    }
}
