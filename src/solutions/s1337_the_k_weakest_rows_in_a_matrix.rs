pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut data: Vec<(Vec<i32>, i32)> = mat
        .into_iter()
        .enumerate()
        .map(|(i, row)| (row, i as i32))
        .collect();
    data.sort_by_key(|(row, _)| row.iter().sum::<i32>());

    data.iter()
        .map(|&(_, index)| index)
        .take(k as usize)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            [2, 0, 3]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            [0, 2]
        );
    }
}
