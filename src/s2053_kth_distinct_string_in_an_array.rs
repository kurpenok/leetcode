use std::collections::HashMap;

pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let unique_strings: HashMap<&String, usize> = arr.iter().fold(HashMap::new(), |mut map, s| {
        *map.entry(s).or_insert(0) += 1;
        map
    });

    arr.iter()
        .filter(|s| unique_strings[*s] == 1)
        .nth(k as usize - 1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string())
}
