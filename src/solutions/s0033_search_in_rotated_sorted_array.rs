pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let min = nums.iter().min().unwrap();
    let min_index = nums.iter().position(|n| n == min).unwrap();

    let part_1 = nums[..min_index].to_vec();
    let part_2 = nums[min_index..].to_vec();

    let mut nums = vec![];
    nums.extend_from_slice(&part_2);
    nums.extend_from_slice(&part_1);

    match nums.binary_search(&target) {
        Ok(index) => ((index + min_index) % nums.len()) as i32,
        _ => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(search(vec![1], 0), -1);
    }
}
