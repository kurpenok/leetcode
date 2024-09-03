pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    for i in 1..matrix.len() {
        if matrix[i - 1][..matrix[0].len() - 1] != matrix[i][1..] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]), false);
    }
}
