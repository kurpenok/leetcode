pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; m];
    matrix[0][0] = 1;

    for i in 0..m {
        for j in 0..n {
            let x = i as i32 - 1;
            let y = j as i32 - 1;

            if x >= 0 && y >= 0 {
                matrix[i][j] = matrix[i][j - 1] + matrix[i - 1][j];
            } else if x < 0 && y >= 0 {
                matrix[i][j] = matrix[i][j - 1];
            } else if x >= 0 && y < 0 {
                matrix[i][j] = matrix[i - 1][j];
            }
        }
    }

    matrix[m - 1][n - 1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(unique_paths(3, 2), 3);
    }
}
