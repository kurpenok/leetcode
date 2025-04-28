pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;

    let mut row_sums: Vec<i32> = vec![0; mat.len()];
    let mut col_sums: Vec<i32> = vec![0; mat[0].len()];

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            row_sums[i] += mat[i][j];
            col_sums[j] += mat[i][j];
        }
    }

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == 1 && row_sums[i] == 1 && col_sums[j] == 1 {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
