pub fn minimum_cost(cost: Vec<i32>) -> i32 {
    let mut cost = cost;
    cost.sort_by(|a, b| b.cmp(a));
    cost.chunks(3)
        .map(|chunk| chunk.iter().take(2).sum::<i32>())
        .sum()
}
