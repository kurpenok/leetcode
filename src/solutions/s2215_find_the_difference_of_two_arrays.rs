use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let nums_1: HashSet<i32> = nums1.iter().copied().collect();
    let nums_2: HashSet<i32> = nums2.iter().copied().collect();

    let mut diff_1: Vec<i32> = nums_1.difference(&nums_2).cloned().collect();
    diff_1.sort();

    let mut diff_2: Vec<i32> = nums_2.difference(&nums_1).cloned().collect();
    diff_2.sort();

    vec![diff_1, diff_2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            [[1, 3], [4, 6]],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]],
        );
    }
}
