use std::cmp::Ordering;

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    if arr.len() < 3 || arr[0] > arr[1] {
        return false;
    }

    let mut rising = true;
    let mut prev = arr[0];

    for i in 1..arr.len() {
        match arr[i].cmp(&prev) {
            Ordering::Greater => {
                if !rising {
                    return false;
                }
            }
            Ordering::Less => rising = false,
            Ordering::Equal => return false,
        }
        prev = arr[i];
    }

    !rising
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(valid_mountain_array(vec![2, 1]), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(valid_mountain_array(vec![3, 5, 5]), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(valid_mountain_array(vec![0, 3, 2, 1]), true);
    }
}
