pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    tickets
        .iter()
        .enumerate()
        .map(|(i, &t)| t.min(tickets[k as usize] - (i > k as usize) as i32))
        .sum()
}
