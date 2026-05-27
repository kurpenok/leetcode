pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    points
        .iter()
        .enumerate()
        .filter(|(_, point)| x == point[0] || y == point[1])
        .min_by_key(|(_, point)| (x - point[0]).abs() + (y - point[1]).abs())
        .map(|(i, _)| i as i32)
        .unwrap_or(-1)
}
