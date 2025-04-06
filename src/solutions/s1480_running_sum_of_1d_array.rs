pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), [1, 3, 6, 10]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), [3, 4, 6, 16, 17]);
    }
}
