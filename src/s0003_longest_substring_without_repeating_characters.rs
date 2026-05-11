use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let symbols: Vec<char> = s.chars().collect();
    let mut slice: HashMap<char, usize> = HashMap::new();

    let mut max_length: usize = 0;
    let mut i: usize = 0;

    while i < symbols.len() {
        if !slice.contains_key(&symbols[i]) {
            slice.insert(symbols[i], i);
            i += 1;
        } else {
            i = slice[&symbols[i]] + 1;
            if max_length < slice.len() {
                max_length = slice.len();
            }
            slice.clear();
        }
    }

    max_length.max(slice.len()) as i32
}
