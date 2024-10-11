pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let l = grid.len() as i32;
    let mut res = 0;

    for (row, i) in grid.iter().zip(0..) {
        for (&val, j) in row.iter().zip(0..).filter(|(val, _)| **val > 0) {
            res += 2;
            for (dx, dy) in dirs.iter().cloned() {
                let (xx, yy) = (j + dx, i + dy);
                res += match xx >= 0 && yy >= 0 && xx < l && yy < l {
                    true => (val - grid[yy as usize][xx as usize]).max(0),
                    false => val,
                };
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            32
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
            46
        );
    }
}
