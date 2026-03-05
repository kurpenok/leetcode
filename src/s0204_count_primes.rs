pub fn count_primes(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }

    let mut is_prime = vec![true; n as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let limit = (n as f64).sqrt() as usize;

    for p in 2..=limit {
        if is_prime[p] {
            let mut m = p * p;
            while m < n as usize {
                is_prime[m] = false;
                m += p;
            }
        }
    }

    is_prime.iter().filter(|&&prime| prime).count() as i32
}
