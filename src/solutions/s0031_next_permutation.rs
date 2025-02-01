pub fn next_permutation(nums: &mut Vec<i32>) {
    if nums.windows(2).all(|n| n[0] >= n[1]) {
        for i in 0..nums.len() / 2 {
            let j = nums.len() - i - 1;
            nums.swap(i, j);
        }
        return;
    } else if nums.windows(2).all(|n| n[0] <= n[1]) {
        let n = nums.len();
        nums.swap(n - 1, n - 2);
        return;
    }

    for i in (1..nums.len()).rev() {
        if nums[i - 1] < nums[i] {
            let mut min = nums[i];
            let mut min_index = i;

            for j in i..nums.len() {
                if nums[j] < min && nums[j] > nums[i - 1] {
                    min = nums[j];
                    min_index = j;
                }
            }

            nums.swap(i - 1, min_index);

            let part = nums[..i].to_vec();
            let mut slice = nums[i..].to_vec();
            slice.sort();

            nums.clear();
            nums.extend_from_slice(&part);
            nums.extend_from_slice(&slice);

            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums: Vec<i32> = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, [1, 3, 2]);
    }

    #[test]
    fn test_case_2() {
        let mut nums: Vec<i32> = vec![3, 2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, [1, 2, 3]);
    }

    #[test]
    fn test_case_3() {
        let mut nums: Vec<i32> = vec![1, 1, 5];
        next_permutation(&mut nums);
        assert_eq!(nums, [1, 5, 1]);
    }
}
