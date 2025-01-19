pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k % (grid.len() * grid[0].len()) as i32;
    if k == 0 {
        return grid;
    }

    let mut grid_elements: Vec<i32> = grid.iter().flatten().cloned().collect();
    grid_elements.rotate_right(k as usize);

    let mut shifted_grid: Vec<Vec<i32>> = Vec::new();
    let mut index = 0;
    for _ in 0..grid.len() {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..grid[0].len() {
            row.push(grid_elements[index]);
            index += 1;
        }
        shifted_grid.push(row);
    }

    shifted_grid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            [[9, 1, 2], [3, 4, 5], [6, 7, 8]],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            shift_grid(
                vec![
                    vec![3, 8, 1, 9],
                    vec![19, 7, 2, 5],
                    vec![4, 6, 11, 10],
                    vec![12, 0, 21, 13],
                ],
                4,
            ),
            [[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]],
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        );
    }
}
