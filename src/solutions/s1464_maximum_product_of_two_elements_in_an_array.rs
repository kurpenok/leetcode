pub fn max_product(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .flat_map(|(i, &x)| nums[i + 1..].iter().map(move |&y| (x - 1) * (y - 1)))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_product(vec![3, 7]), 12);
    }
}
