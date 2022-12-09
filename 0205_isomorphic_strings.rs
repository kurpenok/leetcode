use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s: Vec<_> = s.chars().collect();
        let t: Vec<_> = t.chars().collect();

        let mut words_s: HashMap<_, _> = HashMap::new();
        let mut words_t: HashMap<_, _> = HashMap::new();

        for i in 0..s.len() {
            if words_s.contains_key(&s[i]) {
                if words_s[&s[i]] != t[i] {
                    return false;
                }
            } else {
                words_s.insert(s[i], t[i]);
            }

            if words_t.contains_key(&t[i]) {
                if words_t[&t[i]] != s[i] {
                    return false;
                }
            } else {
                words_t.insert(t[i], s[i]);
            }
        }

        true
    }
}
