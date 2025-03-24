pub fn min_start_value(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, 0), |(sum, min_sum), &n| {
            let new_sum = sum + n;
            (new_sum, min_sum.min(new_sum))
        })
        .1
        .abs()
        + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_start_value(vec![1, 2]), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_start_value(vec![1, -2, -3]), 5);
    }
}
