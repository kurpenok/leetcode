pub fn num_prime_arrangements(n: i32) -> i32 {
    let m: i64 = 1_000_000_007;
    let primes: [i32; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    let factorial_mod = |n: i32| -> i64 { (2..=n as i64).fold(1, |acc, x| (acc * x) % m) };
    let num_primes = primes.iter().take_while(|&&x| x <= n).count() as i32;
    ((factorial_mod(num_primes) * factorial_mod(n - num_primes)) % m) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(num_prime_arrangements(5), 12);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(num_prime_arrangements(100), 682289015);
    }
}
