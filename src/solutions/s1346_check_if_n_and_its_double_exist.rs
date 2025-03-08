pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let mut arr = arr;
    arr.sort();

    for i in 0..arr.len() {
        if arr[i].abs() % 2 == 1 {
            continue;
        }

        match arr.binary_search(&(arr[i] / 2)) {
            Ok(index) => {
                if index != i {
                    return true;
                }
            }
            Err(_) => continue,
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(check_if_exist(vec![10, 2, 5, 3]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_if_exist(vec![3, 1, 7, 11]), false);
    }
}
