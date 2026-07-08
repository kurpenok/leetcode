pub fn max_distance(colors: Vec<i32>) -> i32 {
    let dist_1 = colors.iter().rposition(|&c| c != colors[0]).unwrap_or(0);
    let dist_2 = colors
        .iter()
        .position(|&c| c != colors[colors.len() - 1])
        .map(|i| colors.len() - 1 - i)
        .unwrap_or(0);
    dist_1.max(dist_2) as i32
}
