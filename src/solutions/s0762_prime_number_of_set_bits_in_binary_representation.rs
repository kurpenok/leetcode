use std::collections::HashMap;

fn is_prime(bits_count: i32) -> bool {
    for i in 2..bits_count {
        if bits_count % i == 0 {
            return false;
        }
    }

    true
}

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut primes_counter: usize = 0;

    let mut prime_status: HashMap<i32, bool> = HashMap::new();

    for n in left..=right {
        let bits_count = n.count_ones() as i32;

        if bits_count == 1 {
            continue;
        }

        let status = prime_status.get(&bits_count);

        if status.is_none() {
            prime_status.insert(bits_count, is_prime(bits_count));
        }

        if prime_status[&bits_count] {
            primes_counter += 1;
        }
    }

    primes_counter as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_prime_set_bits(10, 15), 5);
    }
}
