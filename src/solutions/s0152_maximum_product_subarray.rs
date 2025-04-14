pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut min_prod: i32 = nums[0];
    let mut max_prod: i32 = nums[0];
    let mut result: i32 = nums[0];

    for (_, value) in nums.iter().enumerate().skip(1) {
        let test_max_prod: i32 = max_prod * *value;
        let test_min_prod: i32 = min_prod * *value;

        max_prod = test_min_prod.max(test_max_prod).max(*value);
        min_prod = test_max_prod.min(test_min_prod).min(*value);

        result = result.max(max_prod);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }
}
