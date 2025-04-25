pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;

    let n: usize = mat.len();
    for i in 0..n {
        result += mat[i][i] + mat[i][n - i - 1];
    }

    if n % 2 == 1 {
        result -= mat[n / 2][n / 2];
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ]),
            8,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(diagonal_sum(vec![vec![5]]), 5);
    }
}
