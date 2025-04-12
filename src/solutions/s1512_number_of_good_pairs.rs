pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
