pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut highest: i32 = std::cmp::max(nums[0], nums[1]);
    let mut lowest: i32 = std::cmp::min(nums[0], nums[1]);

    let mut highest_product_of_2: i32 = nums[0] * nums[1];
    let mut lowest_product_of_2: i32 = nums[0] * nums[1];

    let mut highest_product_of_3: i32 = nums[0] * nums[1] * nums[2];

    for i in 2..nums.len() {
        highest_product_of_3 = *vec![
            highest_product_of_3,
            nums[i] * highest_product_of_2,
            nums[i] * lowest_product_of_2,
        ]
        .iter()
        .max()
        .unwrap();

        highest_product_of_2 = *vec![highest_product_of_2, nums[i] * highest, nums[i] * lowest]
            .iter()
            .max()
            .unwrap();

        lowest_product_of_2 = *vec![lowest_product_of_2, nums[i] * highest, nums[i] * lowest]
            .iter()
            .min()
            .unwrap();

        highest = std::cmp::max(highest, nums[i]);
        lowest = std::cmp::min(lowest, nums[i]);
    }

    highest_product_of_3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum_product(vec![1, 2, 3]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum_product(vec![1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(maximum_product(vec![-1, -2, -3]), -6);
    }
}
