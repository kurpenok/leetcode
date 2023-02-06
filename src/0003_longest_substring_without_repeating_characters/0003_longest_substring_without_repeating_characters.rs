use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let symbols: Vec<_> = s.chars().collect();
        let mut slice: HashMap<_, usize> = HashMap::new();
        
        let mut max_length: usize = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < symbols.len() {
            if slice.contains_key(&symbols[i]) {
                slice.insert(&symbols[i], i);
                i += 1;
            } else {
                j = slice[&symbols[i]] + 1;
                i = j;
                if max_length < slice.len() {
                    max_length = slice.len();
                }
                slice.clear();
            }
        }
        if max_length < slice.len() {
            max_length = slice.len();
        }

        max_length as i32
    }
}
