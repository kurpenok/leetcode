pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut missing_count: i32 = 0;
    let mut current: i32 = 1;
    let mut index: usize = 0;

    while missing_count < k {
        if index < arr.len() && arr[index] == current {
            index += 1;
        } else {
            missing_count += 1;
            if missing_count == k {
                return current;
            }
        }
        current += 1;
    }

    current - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
