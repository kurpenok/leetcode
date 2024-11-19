pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let total_sum: i32 = arr.iter().sum();
    if total_sum % 3 != 0 {
        return false;
    }

    let target_sum = total_sum / 3;
    let mut current_sum = 0;
    let mut found_parts = 0;

    for num in &arr {
        current_sum += num;
        if current_sum == target_sum {
            found_parts += 1;
            current_sum = 0;
        }
    }

    found_parts >= 3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
            true,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
            false,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
            true,
        );
    }
}
