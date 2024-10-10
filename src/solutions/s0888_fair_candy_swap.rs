use std::cmp::Ordering;

pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let alice_total: i32 = alice_sizes.iter().sum();
    let bob_total: i32 = bob_sizes.iter().sum();
    let avg = (alice_total + bob_total) / 2;
    let diff = (avg - bob_total).abs();

    for num in bob_sizes {
        let diff1 = match bob_total.cmp(&avg) {
            Ordering::Greater => num - diff,
            Ordering::Less => num + diff,
            Ordering::Equal => unreachable!(),
        };
        if alice_sizes.contains(&diff1) {
            return vec![diff1, num];
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(fair_candy_swap(vec![1, 1], vec![2, 2]), [1, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(fair_candy_swap(vec![1, 2], vec![2, 3]), [1, 2]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(fair_candy_swap(vec![2], vec![1, 3]), [2, 3]);
    }
}
