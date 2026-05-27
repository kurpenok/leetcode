pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    items
        .into_iter()
        .filter(|item| {
            item[match rule_key.as_str() {
                "type" => 0,
                "color" => 1,
                "name" => 2,
                _ => unreachable!(),
            }] == rule_value
        })
        .count() as i32
}
