pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    grid.iter().enumerate().fold(0, |ans, (i, v)| {
        ans + v.iter().enumerate().fold(0, |mut y, (j, &x)| {
            if x == 1 {
                y += 4;
                if i > 0 && grid[i - 1][j] == 1 {
                    y -= 2;
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    y -= 2;
                }
            }
            y
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(island_perimeter(vec![vec![1]]), 4);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(island_perimeter(vec![vec![1, 0]]), 4);
    }
}
