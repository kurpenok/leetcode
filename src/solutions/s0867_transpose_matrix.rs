pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matrix_t: Vec<Vec<i32>> = Vec::new();

    for i in 0..matrix[0].len() {
        let mut row_t = Vec::new();
        for j in 0..matrix.len() {
            row_t.push(matrix[j][i]);
        }
        matrix_t.push(row_t);
    }

    matrix_t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            [[1, 4], [2, 5], [3, 6]]
        );
    }
}
