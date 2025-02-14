pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((i32::MIN, 0), |(max_sum, current_sum), &num| {
            let current_sum = std::cmp::max(current_sum + num, num);
            (std::cmp::max(max_sum, current_sum), current_sum)
        })
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
