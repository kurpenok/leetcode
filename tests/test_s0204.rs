use leetcode::s0204_count_primes::count_primes;

#[test]
fn test_case_1() {
    assert_eq!(count_primes(10), 4);
}

#[test]
fn test_case_2() {
    assert_eq!(count_primes(0), 0);
}

#[test]
fn test_case_3() {
    assert_eq!(count_primes(1), 0);
}
