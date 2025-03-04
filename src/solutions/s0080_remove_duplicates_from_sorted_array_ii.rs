fn shift_to_end(nums: &mut Vec<i32>, mut index: usize) {
    for i in index + 1..nums.len() {
        if nums[i] == i32::MAX {
            break;
        }
        nums.swap(index, i);
        index += 1;
    }
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut element: (i32, usize) = (nums[0], 1);

    let mut i: usize = 1;
    while i < nums.len() {
        if nums[i] == i32::MAX {
            break;
        } else if nums[i] != element.0 {
            element.0 = nums[i];
            element.1 = 1;
            i += 1;
        } else if nums[i] == element.0 && element.1 < 2 {
            element.1 += 1;
            i += 1;
        } else if nums[i] == element.0 && element.1 >= 2 {
            nums[i] = i32::MAX;
            shift_to_end(nums, i);
        }
    }

    match nums.iter().position(|&n| n == i32::MAX) {
        Some(index) => index as i32,
        None => nums.len() as i32,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums: Vec<i32> = vec![1, 1, 1, 2, 2, 3];

        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, [1, 1, 2, 2, 3, i32::MAX]);
    }

    #[test]
    fn test_case_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

        assert_eq!(remove_duplicates(&mut nums), 7);
        assert_eq!(nums, [0, 0, 1, 1, 2, 3, 3, i32::MAX, i32::MAX]);
    }
}
