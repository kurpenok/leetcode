pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let mut counter: i32 = 0;

    for i in 0..arr1.len() {
        let mut flag: bool = false;

        for j in 0..arr2.len() {
            if (arr1[i] - arr2[j]).abs() <= d {
                flag = true;
                break;
            }
        }

        if !flag {
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
            1
        );
    }
}
