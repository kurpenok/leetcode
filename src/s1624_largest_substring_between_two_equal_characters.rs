use std::collections::HashMap;

pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut first_seen = HashMap::new();
    let mut max_length = -1;

    for (i, c) in s.char_indices() {
        if let Some(&c_index) = first_seen.get(&c) {
            max_length = max_length.max((i - c_index - 1) as i32);
        } else {
            first_seen.insert(c, i);
        }
    }

    max_length
}
