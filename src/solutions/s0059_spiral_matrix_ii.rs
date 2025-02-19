pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;

    let mut spiral_matrix = vec![vec![0; n]; n];
    let mut count = 1;

    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            spiral_matrix[i][j] = count;
            count += 1;
        }
        for j in i..n - i - 1 {
            spiral_matrix[j][n - i - 1] = count;
            count += 1;
        }
        for j in i..n - i - 1 {
            spiral_matrix[n - i - 1][n - j - 1] = count;
            count += 1;
        }
        for j in i..n - i - 1 {
            spiral_matrix[n - j - 1][i] = count;
            count += 1;
        }
    }

    if n % 2 == 1 {
        spiral_matrix[n >> 1][n >> 1] = count;
    }

    spiral_matrix
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(generate_matrix(3), [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(generate_matrix(1), [[1]]);
    }
}
