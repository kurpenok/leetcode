pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
    let mut s = s.as_str();

    for word in words {
        if let Some(t) = s.strip_prefix(&word) {
            s = t;
            if s.is_empty() {
                return true;
            }
        } else {
            return false;
        }
    }

    false
}
