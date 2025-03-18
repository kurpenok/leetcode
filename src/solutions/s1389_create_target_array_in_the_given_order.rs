pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();

    for (i, &idx) in index.iter().enumerate() {
        array.insert(idx as usize, nums[i]);
    }

    array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            [0, 4, 1, 3, 2]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            [0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(create_target_array(vec![1], vec![0]), [1]);
    }
}
