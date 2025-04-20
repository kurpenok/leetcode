pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;

    while left < right {
        if numbers[left] + numbers[right] == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if numbers[left] + numbers[right] > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [1, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(two_sum(vec![2, 3, 4], 6), [1, 3]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(two_sum(vec![-1, 0], -1), [1, 2]);
    }
}
