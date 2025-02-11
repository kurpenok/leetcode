use std::collections::HashSet;

fn helper(nums: &mut Vec<i32>, left: usize, right: usize, permutations: &mut HashSet<Vec<i32>>) {
    if left == right {
        permutations.insert(nums.clone());
        return;
    }

    for i in left..=right {
        nums.swap(left, i);
        helper(nums, left + 1, right, permutations);
        nums.swap(left, i);
    }
}

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let nums_len = nums.len();

    let mut permutations: HashSet<Vec<i32>> = HashSet::new();

    if nums_len > 0 {
        helper(&mut nums, 0, nums_len - 1, &mut permutations);
    }

    let mut permutations: Vec<Vec<i32>> = permutations.into_iter().collect();
    permutations.sort();
    permutations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            permute_unique(vec![1, 1, 2]),
            [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            permute_unique(vec![1, 2, 3]),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }
}
