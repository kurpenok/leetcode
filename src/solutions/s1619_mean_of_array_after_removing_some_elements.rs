pub fn trim_mean(arr: Vec<i32>) -> f64 {
    let mut arr = arr;
    arr.sort();

    let arr_len = arr.len();
    let arr_len_5per = arr_len / 20;

    let new_arr = &arr[arr_len_5per..(arr_len - arr_len_5per)];
    new_arr.iter().sum::<i32>() as f64 / (arr_len - 2 * arr_len_5per) as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ]),
            2.0,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            trim_mean(vec![
                6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
            ]),
            4.0,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            trim_mean(vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5,
                10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4,
            ]),
            4.777777777777778,
        );
    }
}
