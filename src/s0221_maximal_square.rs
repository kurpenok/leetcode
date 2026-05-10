use std::cmp::min;

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
    let mut max_side = 0;

    for i in 1..matrix.len() + 1 {
        for j in 1..matrix[0].len() + 1 {
            if matrix[i - 1][j - 1] == '1' {
                dp[i][j] = 1 + min(min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]);
                max_side = max_side.max(dp[i][j]);
            }
        }
    }

    max_side * max_side
}
