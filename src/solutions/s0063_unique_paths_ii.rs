pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();

    if obstacle_grid[m - 1][n - 1] == 1 {
        return 0;
    }

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; m];
    matrix[0][0] = 1;

    for i in 0..m {
        for j in 0..n {
            let x = i as i32 - 1;
            let y = j as i32 - 1;

            if x >= 0 && y >= 0 && obstacle_grid[i][j - 1] != 1 && obstacle_grid[i - 1][j] != 1 {
                matrix[i][j] = matrix[i][j - 1] + matrix[i - 1][j];
            } else if x >= 0
                && y >= 0
                && obstacle_grid[i][j - 1] == 1
                && obstacle_grid[i - 1][j] == 1
            {
                matrix[i][j] = 0;
            } else if x >= 0 && y >= 0 && obstacle_grid[i][j - 1] == 1 {
                matrix[i][j] = matrix[i - 1][j];
            } else if x >= 0 && y >= 0 && obstacle_grid[i - 1][j] == 1 {
                matrix[i][j] = matrix[i][j - 1];
            } else if x < 0 && y >= 0 && obstacle_grid[i][j - 1] != 1 {
                matrix[i][j] = matrix[i][j - 1];
            } else if x < 0 && y >= 0 && obstacle_grid[i][j - 1] == 1 {
                matrix[i][j] = 0;
            } else if x >= 0 && y < 0 && obstacle_grid[i - 1][j] != 1 {
                matrix[i][j] = matrix[i - 1][j];
            } else if x >= 0 && y < 0 && obstacle_grid[i - 1][j] == 1 {
                matrix[i][j] = 0;
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
        assert_eq!(
            unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);
    }
}
