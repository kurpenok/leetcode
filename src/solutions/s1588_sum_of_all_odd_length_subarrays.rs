pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for i in (1..=arr.len()).step_by(2) {
        for w in arr.windows(i) {
            sum += w.iter().sum::<i32>();
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
