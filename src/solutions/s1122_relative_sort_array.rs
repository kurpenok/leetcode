pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr1 = Vec::with_capacity(arr1.len());

    let mut arr1 = arr1;
    for n in arr2 {
        loop {
            let n_index = arr1.iter().position(|number| *number == n);
            match n_index {
                Some(n_index) => {
                    sorted_arr1.push(n);
                    arr1.remove(n_index);
                }
                None => break,
            }
        }
    }

    arr1.sort();
    sorted_arr1.extend(arr1);

    sorted_arr1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6],
            ),
            [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            [22, 28, 8, 6, 17, 44],
        );
    }
}
