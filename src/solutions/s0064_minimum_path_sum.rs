pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            let x = i as i32 - 1;
            let y = j as i32 - 1;

            if x >= 0 && y >= 0 {
                matrix[i][j] = std::cmp::min(matrix[i][j - 1], matrix[i - 1][j]);
            } else if x < 0 && y >= 0 {
                matrix[i][j] = matrix[i][j - 1];
            } else if x >= 0 && y < 0 {
                matrix[i][j] = matrix[i - 1][j];
            }

            matrix[i][j] += grid[i][j];
        }
    }

    matrix[m - 1][n - 1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
