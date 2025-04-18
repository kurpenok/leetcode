pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut good_triplets_counter: i32 = 0;

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            for k in j + 1..arr.len() {
                if (arr[i] - arr[j]).abs() <= a
                    && (arr[j] - arr[k]).abs() <= b
                    && (arr[i] - arr[k]).abs() <= c
                {
                    good_triplets_counter += 1;
                }
            }
        }
    }

    good_triplets_counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
}
