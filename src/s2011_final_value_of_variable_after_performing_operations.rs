pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations
        .iter()
        .map(|o| if o.contains('+') { 1 } else { -1 })
        .sum()
}
