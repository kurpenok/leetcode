pub fn mark_island_area(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        if x < rows && y < cols && grid[x][y] == '1' {
            grid[x][y] = '-';

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x + 1 < rows {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y + 1 < cols {
                stack.push((x, y + 1));
            }
        }
    }
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let mut grid = grid;
    let mut islands_counter = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                islands_counter += 1;
                mark_island_area(&mut grid, i, j);
            }
        }
    }

    islands_counter
}
