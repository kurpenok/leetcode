fn helper(i: usize, nums: &Vec<i32>, temp: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if i >= nums.len() {
        return;
    }

    temp.push(nums[i]);
    result.push(temp.clone());
    helper(i + 1, nums, temp, result);
    temp.pop();

    helper(i + 1, nums, temp, result);
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();

    result.push(Vec::new());
    helper(0, &nums, &mut temp, &mut result);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(subsets(vec![0]), vec![vec![], vec![0]]);
    }
}
