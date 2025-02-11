fn helper(nums: &mut Vec<i32>, left: usize, right: usize, permutations: &mut Vec<Vec<i32>>) {
    if left == right {
        permutations.push(nums.clone());
        return;
    }

    for i in left..=right {
        nums.swap(left, i);
        helper(nums, left + 1, right, permutations);
        nums.swap(left, i);
    }
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let nums_len = nums.len();

    let mut permutations: Vec<Vec<i32>> = Vec::new();

    if nums_len > 0 {
        helper(&mut nums, 0, nums_len - 1, &mut permutations);
    }

    permutations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 2, 1],
                [3, 1, 2]
            ]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(permute(vec![0, 1]), [[0, 1], [1, 0]]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(permute(vec![1]), [[1]]);
    }
}
